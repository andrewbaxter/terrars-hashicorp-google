#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IntegrationsAuthConfigData_ {
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
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiry_notification_duration: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_valid_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visibility: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate: std::option::Option<std::vec::Vec<IntegrationsAuthConfig_ClientCertificate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    decrypted_credential: std::option::Option<std::vec::Vec<IntegrationsAuthConfig_DecryptedCredential>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IntegrationsAuthConfig_Timeouts>,
    dynamic: IntegrationsAuthConfigDynamic,
}

struct IntegrationsAuthConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IntegrationsAuthConfigData_>,
}

#[derive(Clone)]
pub struct IntegrationsAuthConfig(std::rc::Rc<IntegrationsAuthConfig_>);

impl IntegrationsAuthConfig {
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

    #[doc = "Set the field `description`.\nA description of the auth config."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expiry_notification_duration`.\nUser can define the time to receive notification after which the auth config becomes invalid. Support up to 30 days. Support granularity in hours.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_expiry_notification_duration(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().expiry_notification_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `override_valid_time`.\nUser provided expiry time to override. For the example of Salesforce, username/password credentials can be valid for 6 months depending on the instance settings.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_override_valid_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().override_valid_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `visibility`.\nThe visibility of the auth config. Possible values: [\"PRIVATE\", \"CLIENT_VISIBLE\"]"]
    pub fn set_visibility(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().visibility = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_certificate`.\n"]
    pub fn set_client_certificate(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationsAuthConfig_ClientCertificate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_certificate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_certificate = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `decrypted_credential`.\n"]
    pub fn set_decrypted_credential(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().decrypted_credential = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.decrypted_credential = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<IntegrationsAuthConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `certificate_id` after provisioning.\nCertificate id for client certificate."]
    pub fn certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the auth config is created.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator_email` after provisioning.\nThe creator's email address. Generated based on the End User Credentials/LOAS role of the user making the call."]
    pub fn creator_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator_email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `credential_type` after provisioning.\nCredential type of the encrypted credential."]
    pub fn credential_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the auth config."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe name of the auth config."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encrypted_credential` after provisioning.\nAuth credential encrypted by Cloud KMS. Can be decrypted as Credential with proper KMS key.\n\nA base64-encoded string."]
    pub fn encrypted_credential(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted_credential", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expiry_notification_duration` after provisioning.\nUser can define the time to receive notification after which the auth config becomes invalid. Support up to 30 days. Support granularity in hours.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn expiry_notification_duration(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expiry_notification_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modifier_email` after provisioning.\nThe last modifier's email address. Generated based on the End User Credentials/LOAS role of the user making the call."]
    pub fn last_modifier_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modifier_email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation in which client needs to be provisioned."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name of the auth config."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `override_valid_time` after provisioning.\nUser provided expiry time to override. For the example of Salesforce, username/password credentials can be valid for 6 months depending on the instance settings.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn override_valid_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_valid_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reason` after provisioning.\nThe reason / details of the current status."]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe status of the auth config."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the auth config is modified.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `valid_time` after provisioning.\nThe time until the auth config is valid. Empty or max value is considered the auth config won't expire.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn valid_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `visibility` after provisioning.\nThe visibility of the auth config. Possible values: [\"PRIVATE\", \"CLIENT_VISIBLE\"]"]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\n"]
    pub fn client_certificate(&self) -> terrars::ListRef<IntegrationsAuthConfig_ClientCertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `decrypted_credential` after provisioning.\n"]
    pub fn decrypted_credential(&self) -> terrars::ListRef<IntegrationsAuthConfig_DecryptedCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.decrypted_credential", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IntegrationsAuthConfig_TimeoutsRef {
        IntegrationsAuthConfig_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for IntegrationsAuthConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IntegrationsAuthConfig { }

impl terrars::ToListMappable for IntegrationsAuthConfig {
    type O = terrars::ListRef<IntegrationsAuthConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IntegrationsAuthConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_integrations_auth_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIntegrationsAuthConfig {
    pub tf_id: std::string::String,
    #[doc = "The name of the auth config."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "Location in which client needs to be provisioned."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationsAuthConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> IntegrationsAuthConfig {
        let out = IntegrationsAuthConfig(std::rc::Rc::new(IntegrationsAuthConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IntegrationsAuthConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                expiry_notification_duration: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                override_valid_time: core::default::Default::default(),
                project: core::default::Default::default(),
                visibility: core::default::Default::default(),
                client_certificate: core::default::Default::default(),
                decrypted_credential: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IntegrationsAuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationsAuthConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IntegrationsAuthConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `certificate_id` after provisioning.\nCertificate id for client certificate."]
    pub fn certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the auth config is created.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator_email` after provisioning.\nThe creator's email address. Generated based on the End User Credentials/LOAS role of the user making the call."]
    pub fn creator_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator_email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `credential_type` after provisioning.\nCredential type of the encrypted credential."]
    pub fn credential_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the auth config."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe name of the auth config."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encrypted_credential` after provisioning.\nAuth credential encrypted by Cloud KMS. Can be decrypted as Credential with proper KMS key.\n\nA base64-encoded string."]
    pub fn encrypted_credential(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted_credential", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expiry_notification_duration` after provisioning.\nUser can define the time to receive notification after which the auth config becomes invalid. Support up to 30 days. Support granularity in hours.\n\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn expiry_notification_duration(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expiry_notification_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modifier_email` after provisioning.\nThe last modifier's email address. Generated based on the End User Credentials/LOAS role of the user making the call."]
    pub fn last_modifier_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modifier_email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation in which client needs to be provisioned."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name of the auth config."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `override_valid_time` after provisioning.\nUser provided expiry time to override. For the example of Salesforce, username/password credentials can be valid for 6 months depending on the instance settings.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn override_valid_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_valid_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reason` after provisioning.\nThe reason / details of the current status."]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe status of the auth config."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the auth config is modified.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `valid_time` after provisioning.\nThe time until the auth config is valid. Empty or max value is considered the auth config won't expire.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn valid_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `visibility` after provisioning.\nThe visibility of the auth config. Possible values: [\"PRIVATE\", \"CLIENT_VISIBLE\"]"]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\n"]
    pub fn client_certificate(&self) -> terrars::ListRef<IntegrationsAuthConfig_ClientCertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `decrypted_credential` after provisioning.\n"]
    pub fn decrypted_credential(&self) -> terrars::ListRef<IntegrationsAuthConfig_DecryptedCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.decrypted_credential", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IntegrationsAuthConfig_TimeoutsRef {
        IntegrationsAuthConfig_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct IntegrationsAuthConfigDynamic {
    client_certificate: std::option::Option<terrars::DynamicBlock<IntegrationsAuthConfig_ClientCertificate>>,
    decrypted_credential: std::option::Option<terrars::DynamicBlock<IntegrationsAuthConfig_DecryptedCredential>>,
}

#[derive(Serialize)]
pub struct IntegrationsAuthConfig_ClientCertificate {
    encrypted_private_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    passphrase: std::option::Option<terrars::PrimField<std::string::String>>,
    ssl_certificate: terrars::PrimField<std::string::String>,
}

impl IntegrationsAuthConfig_ClientCertificate {
    #[doc =
        "Set the field `passphrase`.\n'passphrase' should be left unset if private key is not encrypted.\nNote that 'passphrase' is not the password for web server, but an extra layer of security to protected private key."]
    pub fn set_passphrase(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.passphrase = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationsAuthConfig_ClientCertificate {
    type O = terrars::BlockAssignable<IntegrationsAuthConfig_ClientCertificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationsAuthConfig_ClientCertificate {
    #[doc =
        "The ssl certificate encoded in PEM format. This string must include the begin header and end footer lines."]
    pub encrypted_private_key: terrars::PrimField<std::string::String>,
    #[doc =
        "The ssl certificate encoded in PEM format. This string must include the begin header and end footer lines."]
    pub ssl_certificate: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationsAuthConfig_ClientCertificate {
    pub fn build(self) -> IntegrationsAuthConfig_ClientCertificate {
        IntegrationsAuthConfig_ClientCertificate {
            encrypted_private_key: self.encrypted_private_key,
            passphrase: core::default::Default::default(),
            ssl_certificate: self.ssl_certificate,
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationsAuthConfig_ClientCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationsAuthConfig_ClientCertificateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IntegrationsAuthConfig_ClientCertificateRef {
        IntegrationsAuthConfig_ClientCertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationsAuthConfig_ClientCertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `encrypted_private_key` after provisioning.\nThe ssl certificate encoded in PEM format. This string must include the begin header and end footer lines."]
    pub fn encrypted_private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted_private_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `passphrase` after provisioning.\n'passphrase' should be left unset if private key is not encrypted.\nNote that 'passphrase' is not the password for web server, but an extra layer of security to protected private key."]
    pub fn passphrase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.passphrase", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_certificate` after provisioning.\nThe ssl certificate encoded in PEM format. This string must include the begin header and end footer lines."]
    pub fn ssl_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationsAuthConfig_DecryptedCredential_AuthToken {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationsAuthConfig_DecryptedCredential_AuthToken {
    #[doc = "Set the field `token`.\nThe token for the auth type."]
    pub fn set_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nAuthentication type, e.g. \"Basic\", \"Bearer\", etc."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationsAuthConfig_DecryptedCredential_AuthToken {
    type O = terrars::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential_AuthToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationsAuthConfig_DecryptedCredential_AuthToken {}

impl BuildIntegrationsAuthConfig_DecryptedCredential_AuthToken {
    pub fn build(self) -> IntegrationsAuthConfig_DecryptedCredential_AuthToken {
        IntegrationsAuthConfig_DecryptedCredential_AuthToken {
            token: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationsAuthConfig_DecryptedCredential_AuthTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationsAuthConfig_DecryptedCredential_AuthTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationsAuthConfig_DecryptedCredential_AuthTokenRef {
        IntegrationsAuthConfig_DecryptedCredential_AuthTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationsAuthConfig_DecryptedCredential_AuthTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\nThe token for the auth type."]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAuthentication type, e.g. \"Basic\", \"Bearer\", etc."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationsAuthConfig_DecryptedCredential_Jwt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt_header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt_payload: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationsAuthConfig_DecryptedCredential_Jwt {
    #[doc = "Set the field `jwt_header`.\nIdentifies which algorithm is used to generate the signature."]
    pub fn set_jwt_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.jwt_header = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jwt_payload`.\nContains a set of claims. The JWT specification defines seven Registered Claim Names which are the standard fields commonly included in tokens. Custom claims are usually also included, depending on the purpose of the token."]
    pub fn set_jwt_payload(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.jwt_payload = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret`.\nUser's pre-shared secret to sign the token."]
    pub fn set_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationsAuthConfig_DecryptedCredential_Jwt {
    type O = terrars::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential_Jwt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationsAuthConfig_DecryptedCredential_Jwt {}

impl BuildIntegrationsAuthConfig_DecryptedCredential_Jwt {
    pub fn build(self) -> IntegrationsAuthConfig_DecryptedCredential_Jwt {
        IntegrationsAuthConfig_DecryptedCredential_Jwt {
            jwt_header: core::default::Default::default(),
            jwt_payload: core::default::Default::default(),
            secret: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationsAuthConfig_DecryptedCredential_JwtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationsAuthConfig_DecryptedCredential_JwtRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationsAuthConfig_DecryptedCredential_JwtRef {
        IntegrationsAuthConfig_DecryptedCredential_JwtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationsAuthConfig_DecryptedCredential_JwtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `jwt` after provisioning.\nThe token calculated by the header, payload and signature."]
    pub fn jwt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwt", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jwt_header` after provisioning.\nIdentifies which algorithm is used to generate the signature."]
    pub fn jwt_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwt_header", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jwt_payload` after provisioning.\nContains a set of claims. The JWT specification defines seven Registered Claim Names which are the standard fields commonly included in tokens. Custom claims are usually also included, depending on the purpose of the token."]
    pub fn jwt_payload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwt_payload", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret` after provisioning.\nUser's pre-shared secret to sign the token."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationsAuthConfig_DecryptedCredential_Oauth2AuthorizationCode {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationsAuthConfig_DecryptedCredential_Oauth2AuthorizationCode {
    #[doc = "Set the field `auth_endpoint`.\nThe auth url endpoint to send the auth code request to."]
    pub fn set_auth_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\nThe client's id."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\nThe client's secret."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\nA space-delimited list of requested scope permissions."]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\nThe token url endpoint to send the token request to."]
    pub fn set_token_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationsAuthConfig_DecryptedCredential_Oauth2AuthorizationCode {
    type O = terrars::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential_Oauth2AuthorizationCode>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationsAuthConfig_DecryptedCredential_Oauth2AuthorizationCode {}

impl BuildIntegrationsAuthConfig_DecryptedCredential_Oauth2AuthorizationCode {
    pub fn build(self) -> IntegrationsAuthConfig_DecryptedCredential_Oauth2AuthorizationCode {
        IntegrationsAuthConfig_DecryptedCredential_Oauth2AuthorizationCode {
            auth_endpoint: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            scope: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationsAuthConfig_DecryptedCredential_Oauth2AuthorizationCodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationsAuthConfig_DecryptedCredential_Oauth2AuthorizationCodeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationsAuthConfig_DecryptedCredential_Oauth2AuthorizationCodeRef {
        IntegrationsAuthConfig_DecryptedCredential_Oauth2AuthorizationCodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationsAuthConfig_DecryptedCredential_Oauth2AuthorizationCodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_endpoint` after provisioning.\nThe auth url endpoint to send the auth code request to."]
    pub fn auth_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe client's id."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nThe client's secret."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nA space-delimited list of requested scope permissions."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token_endpoint` after provisioning.\nThe token url endpoint to send the token request to."]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValue {
    #[doc = "Set the field `string_value`.\nString."]
    pub fn set_string_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValue {
    type O =
        terrars
        ::BlockAssignable<
            IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValue {}

impl BuildIntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValue {
    pub fn build(
        self,
    ) -> IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValue {
        IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValue {
            string_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValueRef {
        IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `string_value` after provisioning.\nString."]
    pub fn string_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_KeyDynamic {
    literal_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    literal_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValue>,
    >,
    dynamic: IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_KeyDynamic,
}

impl IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key {
    #[doc = "Set the field `literal_value`.\n"]
    pub fn set_literal_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.literal_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.literal_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key {
    type O =
        terrars
        ::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key {}

impl BuildIntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key {
    pub fn build(self) -> IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key {
        IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key {
            literal_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_KeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_KeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_KeyRef {
        IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_KeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_KeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `literal_value` after provisioning.\n"]
    pub fn literal_value(
        &self,
    ) -> terrars
    ::ListRef<
        IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.literal_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_ValueDynamic {
    literal_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Value {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    literal_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValue>,
    >,
    dynamic: IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_ValueDynamic,
}

impl IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Value {
    #[doc = "Set the field `literal_value`.\n"]
    pub fn set_literal_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.literal_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.literal_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Value {
    type O =
        terrars
        ::BlockAssignable<
            IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Value,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Value {}

impl BuildIntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Value {
    pub fn build(
        self,
    ) -> IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Value {
        IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Value {
            literal_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_ValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_ValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_ValueRef {
        IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_ValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_ValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `literal_value` after provisioning.\n"]
    pub fn literal_value(
        &self,
    ) -> terrars
    ::ListRef<
        IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key_LiteralValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.literal_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_EntriesDynamic {
    key: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key>,
    >,
    value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Value>,
    >,
}

#[derive(Serialize)]
pub struct IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std
    ::option
    ::Option<
        std::vec::Vec<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std
    ::option
    ::Option<
        std::vec::Vec<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Value>,
    >,
    dynamic: IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_EntriesDynamic,
}

impl IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Key,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_Value,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries {
    type O =
        terrars
        ::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries {}

impl BuildIntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries {
    pub fn build(self) -> IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries {
        IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_EntriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_EntriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_EntriesRef {
        IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_EntriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_EntriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(
        &self,
    ) -> terrars
    ::ListRef<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_KeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(
        &self,
    ) -> terrars
    ::ListRef<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries_ValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParamsDynamic {
    entries: std
    ::option
    ::Option<
        terrars::DynamicBlock<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries>,
    >,
}

#[derive(Serialize)]
pub struct IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entries: std
    ::option
    ::Option<std::vec::Vec<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries>>,
    dynamic: IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParamsDynamic,
}

impl IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams {
    #[doc = "Set the field `entries`.\n"]
    pub fn set_entries(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_Entries,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.entries = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.entries = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams {
    type O =
        terrars::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams {}

impl BuildIntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams {
    pub fn build(self) -> IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams {
        IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams {
            entries: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParamsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParamsRef {
        IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entries` after provisioning.\n"]
    pub fn entries(
        &self,
    ) -> terrars::ListRef<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams_EntriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entries", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentialsDynamic {
    token_params: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams>>,
}

#[derive(Serialize)]
pub struct IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_params: std
    ::option
    ::Option<std::vec::Vec<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams>>,
    dynamic: IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentialsDynamic,
}

impl IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials {
    #[doc = "Set the field `client_id`.\nThe client's ID."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\nThe client's secret."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `request_type`.\nRepresent how to pass parameters to fetch access token Possible values: [\"REQUEST_TYPE_UNSPECIFIED\", \"REQUEST_BODY\", \"QUERY_PARAMETERS\", \"ENCODED_HEADER\"]"]
    pub fn set_request_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.request_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\nA space-delimited list of requested scope permissions."]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `token_endpoint`.\nThe token endpoint is used by the client to obtain an access token by presenting its authorization grant or refresh token."]
    pub fn set_token_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_params`.\n"]
    pub fn set_token_params(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParams,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.token_params = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.token_params = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials {
    type O = terrars::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials {}

impl BuildIntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials {
    pub fn build(self) -> IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials {
        IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials {
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            request_type: core::default::Default::default(),
            scope: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
            token_params: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentialsRef {
        IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe client's ID."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nThe client's secret."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_type` after provisioning.\nRepresent how to pass parameters to fetch access token Possible values: [\"REQUEST_TYPE_UNSPECIFIED\", \"REQUEST_BODY\", \"QUERY_PARAMETERS\", \"ENCODED_HEADER\"]"]
    pub fn request_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nA space-delimited list of requested scope permissions."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token_endpoint` after provisioning.\nThe token endpoint is used by the client to obtain an access token by presenting its authorization grant or refresh token."]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `token_params` after provisioning.\n"]
    pub fn token_params(
        &self,
    ) -> terrars::ListRef<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials_TokenParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.token_params", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationsAuthConfig_DecryptedCredential_OidcToken {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audience: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationsAuthConfig_DecryptedCredential_OidcToken {
    #[doc =
        "Set the field `audience`.\nAudience to be used when generating OIDC token. The audience claim identifies the recipients that the JWT is intended for."]
    pub fn set_audience(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audience = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account_email`.\nThe service account email to be used as the identity for the token."]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationsAuthConfig_DecryptedCredential_OidcToken {
    type O = terrars::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential_OidcToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationsAuthConfig_DecryptedCredential_OidcToken {}

impl BuildIntegrationsAuthConfig_DecryptedCredential_OidcToken {
    pub fn build(self) -> IntegrationsAuthConfig_DecryptedCredential_OidcToken {
        IntegrationsAuthConfig_DecryptedCredential_OidcToken {
            audience: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationsAuthConfig_DecryptedCredential_OidcTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationsAuthConfig_DecryptedCredential_OidcTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationsAuthConfig_DecryptedCredential_OidcTokenRef {
        IntegrationsAuthConfig_DecryptedCredential_OidcTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationsAuthConfig_DecryptedCredential_OidcTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `audience` after provisioning.\nAudience to be used when generating OIDC token. The audience claim identifies the recipients that the JWT is intended for."]
    pub fn audience(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audience", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nThe service account email to be used as the identity for the token."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token` after provisioning.\nID token obtained for the service account."]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token_expire_time` after provisioning.\nThe approximate time until the token retrieved is valid.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn token_expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_expire_time", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationsAuthConfig_DecryptedCredential_ServiceAccountCredentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationsAuthConfig_DecryptedCredential_ServiceAccountCredentials {
    #[doc = "Set the field `scope`.\nA space-delimited list of requested scope permissions."]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nName of the service account that has the permission to make the request."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationsAuthConfig_DecryptedCredential_ServiceAccountCredentials {
    type O = terrars::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential_ServiceAccountCredentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationsAuthConfig_DecryptedCredential_ServiceAccountCredentials {}

impl BuildIntegrationsAuthConfig_DecryptedCredential_ServiceAccountCredentials {
    pub fn build(self) -> IntegrationsAuthConfig_DecryptedCredential_ServiceAccountCredentials {
        IntegrationsAuthConfig_DecryptedCredential_ServiceAccountCredentials {
            scope: core::default::Default::default(),
            service_account: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationsAuthConfig_DecryptedCredential_ServiceAccountCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationsAuthConfig_DecryptedCredential_ServiceAccountCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationsAuthConfig_DecryptedCredential_ServiceAccountCredentialsRef {
        IntegrationsAuthConfig_DecryptedCredential_ServiceAccountCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationsAuthConfig_DecryptedCredential_ServiceAccountCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nA space-delimited list of requested scope permissions."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nName of the service account that has the permission to make the request."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationsAuthConfig_DecryptedCredential_UsernameAndPassword {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationsAuthConfig_DecryptedCredential_UsernameAndPassword {
    #[doc = "Set the field `password`.\nPassword to be used."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\nUsername to be used."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationsAuthConfig_DecryptedCredential_UsernameAndPassword {
    type O = terrars::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential_UsernameAndPassword>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationsAuthConfig_DecryptedCredential_UsernameAndPassword {}

impl BuildIntegrationsAuthConfig_DecryptedCredential_UsernameAndPassword {
    pub fn build(self) -> IntegrationsAuthConfig_DecryptedCredential_UsernameAndPassword {
        IntegrationsAuthConfig_DecryptedCredential_UsernameAndPassword {
            password: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationsAuthConfig_DecryptedCredential_UsernameAndPasswordRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationsAuthConfig_DecryptedCredential_UsernameAndPasswordRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationsAuthConfig_DecryptedCredential_UsernameAndPasswordRef {
        IntegrationsAuthConfig_DecryptedCredential_UsernameAndPasswordRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationsAuthConfig_DecryptedCredential_UsernameAndPasswordRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword to be used."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername to be used."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationsAuthConfig_DecryptedCredentialDynamic {
    auth_token: std::option::Option<terrars::DynamicBlock<IntegrationsAuthConfig_DecryptedCredential_AuthToken>>,
    jwt: std::option::Option<terrars::DynamicBlock<IntegrationsAuthConfig_DecryptedCredential_Jwt>>,
    oauth2_authorization_code: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationsAuthConfig_DecryptedCredential_Oauth2AuthorizationCode>>,
    oauth2_client_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials>>,
    oidc_token: std::option::Option<terrars::DynamicBlock<IntegrationsAuthConfig_DecryptedCredential_OidcToken>>,
    service_account_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationsAuthConfig_DecryptedCredential_ServiceAccountCredentials>>,
    username_and_password: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationsAuthConfig_DecryptedCredential_UsernameAndPassword>>,
}

#[derive(Serialize)]
pub struct IntegrationsAuthConfig_DecryptedCredential {
    credential_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_token: std::option::Option<std::vec::Vec<IntegrationsAuthConfig_DecryptedCredential_AuthToken>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt: std::option::Option<std::vec::Vec<IntegrationsAuthConfig_DecryptedCredential_Jwt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_authorization_code: std
    ::option
    ::Option<std::vec::Vec<IntegrationsAuthConfig_DecryptedCredential_Oauth2AuthorizationCode>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_credentials: std
    ::option
    ::Option<std::vec::Vec<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc_token: std::option::Option<std::vec::Vec<IntegrationsAuthConfig_DecryptedCredential_OidcToken>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_credentials: std
    ::option
    ::Option<std::vec::Vec<IntegrationsAuthConfig_DecryptedCredential_ServiceAccountCredentials>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username_and_password: std
    ::option
    ::Option<std::vec::Vec<IntegrationsAuthConfig_DecryptedCredential_UsernameAndPassword>>,
    dynamic: IntegrationsAuthConfig_DecryptedCredentialDynamic,
}

impl IntegrationsAuthConfig_DecryptedCredential {
    #[doc = "Set the field `auth_token`.\n"]
    pub fn set_auth_token(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential_AuthToken>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auth_token = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auth_token = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jwt`.\n"]
    pub fn set_jwt(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential_Jwt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.jwt = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.jwt = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth2_authorization_code`.\n"]
    pub fn set_oauth2_authorization_code(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential_Oauth2AuthorizationCode>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth2_authorization_code = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2_authorization_code = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth2_client_credentials`.\n"]
    pub fn set_oauth2_client_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentials>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth2_client_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2_client_credentials = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oidc_token`.\n"]
    pub fn set_oidc_token(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential_OidcToken>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oidc_token = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oidc_token = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_account_credentials`.\n"]
    pub fn set_service_account_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential_ServiceAccountCredentials>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_account_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_account_credentials = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `username_and_password`.\n"]
    pub fn set_username_and_password(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential_UsernameAndPassword>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.username_and_password = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.username_and_password = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IntegrationsAuthConfig_DecryptedCredential {
    type O = terrars::BlockAssignable<IntegrationsAuthConfig_DecryptedCredential>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationsAuthConfig_DecryptedCredential {
    #[doc = "Credential type associated with auth configs."]
    pub credential_type: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationsAuthConfig_DecryptedCredential {
    pub fn build(self) -> IntegrationsAuthConfig_DecryptedCredential {
        IntegrationsAuthConfig_DecryptedCredential {
            credential_type: self.credential_type,
            auth_token: core::default::Default::default(),
            jwt: core::default::Default::default(),
            oauth2_authorization_code: core::default::Default::default(),
            oauth2_client_credentials: core::default::Default::default(),
            oidc_token: core::default::Default::default(),
            service_account_credentials: core::default::Default::default(),
            username_and_password: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationsAuthConfig_DecryptedCredentialRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationsAuthConfig_DecryptedCredentialRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IntegrationsAuthConfig_DecryptedCredentialRef {
        IntegrationsAuthConfig_DecryptedCredentialRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationsAuthConfig_DecryptedCredentialRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `credential_type` after provisioning.\nCredential type associated with auth configs."]
    pub fn credential_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_type", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_token` after provisioning.\n"]
    pub fn auth_token(&self) -> terrars::ListRef<IntegrationsAuthConfig_DecryptedCredential_AuthTokenRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_token", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt` after provisioning.\n"]
    pub fn jwt(&self) -> terrars::ListRef<IntegrationsAuthConfig_DecryptedCredential_JwtRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jwt", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_authorization_code` after provisioning.\n"]
    pub fn oauth2_authorization_code(
        &self,
    ) -> terrars::ListRef<IntegrationsAuthConfig_DecryptedCredential_Oauth2AuthorizationCodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_authorization_code", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_client_credentials` after provisioning.\n"]
    pub fn oauth2_client_credentials(
        &self,
    ) -> terrars::ListRef<IntegrationsAuthConfig_DecryptedCredential_Oauth2ClientCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_client_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `oidc_token` after provisioning.\n"]
    pub fn oidc_token(&self) -> terrars::ListRef<IntegrationsAuthConfig_DecryptedCredential_OidcTokenRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc_token", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_credentials` after provisioning.\n"]
    pub fn service_account_credentials(
        &self,
    ) -> terrars::ListRef<IntegrationsAuthConfig_DecryptedCredential_ServiceAccountCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `username_and_password` after provisioning.\n"]
    pub fn username_and_password(
        &self,
    ) -> terrars::ListRef<IntegrationsAuthConfig_DecryptedCredential_UsernameAndPasswordRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.username_and_password", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationsAuthConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationsAuthConfig_Timeouts {
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

impl terrars::ToListMappable for IntegrationsAuthConfig_Timeouts {
    type O = terrars::BlockAssignable<IntegrationsAuthConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationsAuthConfig_Timeouts {}

impl BuildIntegrationsAuthConfig_Timeouts {
    pub fn build(self) -> IntegrationsAuthConfig_Timeouts {
        IntegrationsAuthConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationsAuthConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationsAuthConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IntegrationsAuthConfig_TimeoutsRef {
        IntegrationsAuthConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationsAuthConfig_TimeoutsRef {
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
