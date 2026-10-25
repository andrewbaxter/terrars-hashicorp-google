#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IamWorkforcePoolProviderData_ {
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
    attribute_condition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_mapping: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detailed_audit_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    provider_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scim_usage: std::option::Option<terrars::PrimField<std::string::String>>,
    workforce_pool_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extended_attributes_oauth2_client: std
    ::option
    ::Option<std::vec::Vec<IamWorkforcePoolProvider_ExtendedAttributesOauth2Client>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extra_attributes_oauth2_client: std
    ::option
    ::Option<std::vec::Vec<IamWorkforcePoolProvider_ExtraAttributesOauth2Client>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc: std::option::Option<std::vec::Vec<IamWorkforcePoolProvider_Oidc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saml: std::option::Option<std::vec::Vec<IamWorkforcePoolProvider_Saml>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IamWorkforcePoolProvider_Timeouts>,
    dynamic: IamWorkforcePoolProviderDynamic,
}

struct IamWorkforcePoolProvider_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IamWorkforcePoolProviderData_>,
}

#[derive(Clone)]
pub struct IamWorkforcePoolProvider(std::rc::Rc<IamWorkforcePoolProvider_>);

impl IamWorkforcePoolProvider {
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
        "Set the field `attribute_condition`.\nA [Common Expression Language](https://github.com/google/cel-spec) expression, in\nplain text, to restrict what otherwise valid authentication credentials issued by the\nprovider should not be accepted.\n\nThe expression must output a boolean representing whether to allow the federation.\n\nThe following keywords may be referenced in the expressions:\n  * 'assertion': JSON representing the authentication credential issued by the provider.\n  * 'google': The Google attributes mapped from the assertion in the 'attribute_mappings'.\n    'google.profile_photo' and 'google.display_name' are not supported.\n  * 'attribute': The custom attributes mapped from the assertion in the 'attribute_mappings'.\n\nThe maximum length of the attribute condition expression is 4096 characters.\nIf unspecified, all valid authentication credentials will be accepted.\n\nThe following example shows how to only allow credentials with a mapped 'google.groups' value of 'admins':\n'''\n\"'admins' in google.groups\"\n'''"]
    pub fn set_attribute_condition(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().attribute_condition = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `attribute_mapping`.\nMaps attributes from the authentication credentials issued by an external identity provider\nto Google Cloud attributes, such as 'subject' and 'segment'.\n\nEach key must be a string specifying the Google Cloud IAM attribute to map to.\n\nThe following keys are supported:\n  * 'google.subject': The principal IAM is authenticating. You can reference this value in IAM bindings.\n    This is also the subject that appears in Cloud Logging logs. This is a required field and\n    the mapped subject cannot exceed 127 bytes.\n  * 'google.groups': Groups the authenticating user belongs to. You can grant groups access to\n    resources using an IAM 'principalSet' binding; access applies to all members of the group.\n  * 'google.display_name': The name of the authenticated user. This is an optional field and\n    the mapped display name cannot exceed 100 bytes. If not set, 'google.subject' will be displayed instead.\n    This attribute cannot be referenced in IAM bindings.\n  * 'google.profile_photo': The URL that specifies the authenticated user's thumbnail photo.\n    This is an optional field. When set, the image will be visible as the user's profile picture.\n    If not set, a generic user icon will be displayed instead.\n    This attribute cannot be referenced in IAM bindings.\n\nYou can also provide custom attributes by specifying 'attribute.{custom_attribute}', where {custom_attribute}\nis the name of the custom attribute to be mapped. You can define a maximum of 50 custom attributes.\nThe maximum length of a mapped attribute key is 100 characters, and the key may only contain the characters [a-z0-9_].\n\nYou can reference these attributes in IAM policies to define fine-grained access for a workforce pool\nto Google Cloud resources. For example:\n  * 'google.subject':\n    'principal://iam.googleapis.com/locations/{location}/workforcePools/{pool}/subject/{value}'\n  * 'google.groups':\n    'principalSet://iam.googleapis.com/locations/{location}/workforcePools/{pool}/group/{value}'\n  * 'attribute.{custom_attribute}':\n    'principalSet://iam.googleapis.com/locations/{location}/workforcePools/{pool}/attribute.{custom_attribute}/{value}'\n\nEach value must be a [Common Expression Language](https://github.com/google/cel-spec)\nfunction that maps an identity provider credential to the normalized attribute specified\nby the corresponding map key.\n\nYou can use the 'assertion' keyword in the expression to access a JSON representation of\nthe authentication credential issued by the provider.\n\nThe maximum length of an attribute mapping expression is 2048 characters. When evaluated,\nthe total size of all mapped attributes must not exceed 8KB.\n\nFor OIDC providers, you must supply a custom mapping that includes the 'google.subject' attribute.\nFor example, the following maps the sub claim of the incoming credential to the 'subject' attribute\non a Google token:\n'''\n{\"google.subject\": \"assertion.sub\"}\n'''\n\nAn object containing a list of '\"key\": value' pairs.\nExample: '{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }'."]
    pub fn set_attribute_mapping(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().attribute_mapping = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA user-specified description of the provider. Cannot exceed 256 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `detailed_audit_logging`.\nIf true, populates additional debug information in Cloud Audit Logs for this provider. Logged attribute mappings and values can be found in 'sts.googleapis.com' data access logs. Default value is false."]
    pub fn set_detailed_audit_logging(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().detailed_audit_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nWhether the provider is disabled. You cannot use a disabled provider to exchange tokens.\nHowever, existing tokens still grant access."]
    pub fn set_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `display_name`.\nA user-specified display name for the provider. Cannot exceed 32 characters."]
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
        "Set the field `scim_usage`.\nAgentspace only. Specifies whether the workforce identity pool\nprovider uses SCIM-managed groups instead of the 'google.groups'\nattribute mapping for authorization checks.\n\nThe 'scimUsage' and 'extendedAttributesOauth2Client' fields are\nmutually exclusive. A request that enables both fields on the same\nworkforce identity pool provider will produce an error.\n* SCIM_USAGE_UNSPECIFIED: Default behaviour\n* ENABLED_FOR_GROUPS: Use SCIM-managed groups instead of the 'google.groups'\n  attribute mapping for authorization checks Possible values: [\"SCIM_USAGE_UNSPECIFIED\", \"ENABLED_FOR_GROUPS\"]"]
    pub fn set_scim_usage(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().scim_usage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `extended_attributes_oauth2_client`.\n"]
    pub fn set_extended_attributes_oauth2_client(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IamWorkforcePoolProvider_ExtendedAttributesOauth2Client>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().extended_attributes_oauth2_client = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.extended_attributes_oauth2_client = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `extra_attributes_oauth2_client`.\n"]
    pub fn set_extra_attributes_oauth2_client(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamWorkforcePoolProvider_ExtraAttributesOauth2Client>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().extra_attributes_oauth2_client = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.extra_attributes_oauth2_client = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oidc`.\n"]
    pub fn set_oidc(self, v: impl std::convert::Into<terrars::BlockAssignable<IamWorkforcePoolProvider_Oidc>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oidc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oidc = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `saml`.\n"]
    pub fn set_saml(self, v: impl std::convert::Into<terrars::BlockAssignable<IamWorkforcePoolProvider_Saml>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().saml = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.saml = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<IamWorkforcePoolProvider_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `attribute_condition` after provisioning.\nA [Common Expression Language](https://github.com/google/cel-spec) expression, in\nplain text, to restrict what otherwise valid authentication credentials issued by the\nprovider should not be accepted.\n\nThe expression must output a boolean representing whether to allow the federation.\n\nThe following keywords may be referenced in the expressions:\n  * 'assertion': JSON representing the authentication credential issued by the provider.\n  * 'google': The Google attributes mapped from the assertion in the 'attribute_mappings'.\n    'google.profile_photo' and 'google.display_name' are not supported.\n  * 'attribute': The custom attributes mapped from the assertion in the 'attribute_mappings'.\n\nThe maximum length of the attribute condition expression is 4096 characters.\nIf unspecified, all valid authentication credentials will be accepted.\n\nThe following example shows how to only allow credentials with a mapped 'google.groups' value of 'admins':\n'''\n\"'admins' in google.groups\"\n'''"]
    pub fn attribute_condition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_condition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `attribute_mapping` after provisioning.\nMaps attributes from the authentication credentials issued by an external identity provider\nto Google Cloud attributes, such as 'subject' and 'segment'.\n\nEach key must be a string specifying the Google Cloud IAM attribute to map to.\n\nThe following keys are supported:\n  * 'google.subject': The principal IAM is authenticating. You can reference this value in IAM bindings.\n    This is also the subject that appears in Cloud Logging logs. This is a required field and\n    the mapped subject cannot exceed 127 bytes.\n  * 'google.groups': Groups the authenticating user belongs to. You can grant groups access to\n    resources using an IAM 'principalSet' binding; access applies to all members of the group.\n  * 'google.display_name': The name of the authenticated user. This is an optional field and\n    the mapped display name cannot exceed 100 bytes. If not set, 'google.subject' will be displayed instead.\n    This attribute cannot be referenced in IAM bindings.\n  * 'google.profile_photo': The URL that specifies the authenticated user's thumbnail photo.\n    This is an optional field. When set, the image will be visible as the user's profile picture.\n    If not set, a generic user icon will be displayed instead.\n    This attribute cannot be referenced in IAM bindings.\n\nYou can also provide custom attributes by specifying 'attribute.{custom_attribute}', where {custom_attribute}\nis the name of the custom attribute to be mapped. You can define a maximum of 50 custom attributes.\nThe maximum length of a mapped attribute key is 100 characters, and the key may only contain the characters [a-z0-9_].\n\nYou can reference these attributes in IAM policies to define fine-grained access for a workforce pool\nto Google Cloud resources. For example:\n  * 'google.subject':\n    'principal://iam.googleapis.com/locations/{location}/workforcePools/{pool}/subject/{value}'\n  * 'google.groups':\n    'principalSet://iam.googleapis.com/locations/{location}/workforcePools/{pool}/group/{value}'\n  * 'attribute.{custom_attribute}':\n    'principalSet://iam.googleapis.com/locations/{location}/workforcePools/{pool}/attribute.{custom_attribute}/{value}'\n\nEach value must be a [Common Expression Language](https://github.com/google/cel-spec)\nfunction that maps an identity provider credential to the normalized attribute specified\nby the corresponding map key.\n\nYou can use the 'assertion' keyword in the expression to access a JSON representation of\nthe authentication credential issued by the provider.\n\nThe maximum length of an attribute mapping expression is 2048 characters. When evaluated,\nthe total size of all mapped attributes must not exceed 8KB.\n\nFor OIDC providers, you must supply a custom mapping that includes the 'google.subject' attribute.\nFor example, the following maps the sub claim of the incoming credential to the 'subject' attribute\non a Google token:\n'''\n{\"google.subject\": \"assertion.sub\"}\n'''\n\nAn object containing a list of '\"key\": value' pairs.\nExample: '{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }'."]
    pub fn attribute_mapping(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.attribute_mapping", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA user-specified description of the provider. Cannot exceed 256 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `detailed_audit_logging` after provisioning.\nIf true, populates additional debug information in Cloud Audit Logs for this provider. Logged attribute mappings and values can be found in 'sts.googleapis.com' data access logs. Default value is false."]
    pub fn detailed_audit_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detailed_audit_logging", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the provider is disabled. You cannot use a disabled provider to exchange tokens.\nHowever, existing tokens still grant access."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA user-specified display name for the provider. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. The resource name of the provider.\nFormat: 'locations/{location}/workforcePools/{workforcePoolId}/providers/{providerId}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provider_id` after provisioning.\nThe ID for the provider, which becomes the final component of the resource name.\nThis value must be 4-32 characters, and may contain the characters [a-z0-9-].\nThe prefix 'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scim_usage` after provisioning.\nAgentspace only. Specifies whether the workforce identity pool\nprovider uses SCIM-managed groups instead of the 'google.groups'\nattribute mapping for authorization checks.\n\nThe 'scimUsage' and 'extendedAttributesOauth2Client' fields are\nmutually exclusive. A request that enables both fields on the same\nworkforce identity pool provider will produce an error.\n* SCIM_USAGE_UNSPECIFIED: Default behaviour\n* ENABLED_FOR_GROUPS: Use SCIM-managed groups instead of the 'google.groups'\n  attribute mapping for authorization checks Possible values: [\"SCIM_USAGE_UNSPECIFIED\", \"ENABLED_FOR_GROUPS\"]"]
    pub fn scim_usage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scim_usage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the provider.\n* STATE_UNSPECIFIED: State unspecified.\n* ACTIVE: The provider is active and may be used to validate authentication credentials.\n* DELETED: The provider is soft-deleted. Soft-deleted providers are permanently\n  deleted after approximately 30 days. You can restore a soft-deleted provider using\n  [providers.undelete](https://cloud.google.com/iam/docs/reference/rest/v1/locations.workforcePools.providers/undelete#google.iam.admin.v1.WorkforcePools.UndeleteWorkforcePoolProvider)."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workforce_pool_id` after provisioning.\nThe ID to use for the pool, which becomes the final component of the resource name.\nThe IDs must be a globally unique string of 6 to 63 lowercase letters, digits, or hyphens.\nIt must start with a letter, and cannot have a trailing hyphen.\nThe prefix 'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workforce_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workforce_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `extended_attributes_oauth2_client` after provisioning.\n"]
    pub fn extended_attributes_oauth2_client(
        &self,
    ) -> terrars::ListRef<IamWorkforcePoolProvider_ExtendedAttributesOauth2ClientRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.extended_attributes_oauth2_client", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `extra_attributes_oauth2_client` after provisioning.\n"]
    pub fn extra_attributes_oauth2_client(
        &self,
    ) -> terrars::ListRef<IamWorkforcePoolProvider_ExtraAttributesOauth2ClientRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.extra_attributes_oauth2_client", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> terrars::ListRef<IamWorkforcePoolProvider_OidcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> terrars::ListRef<IamWorkforcePoolProvider_SamlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkforcePoolProvider_TimeoutsRef {
        IamWorkforcePoolProvider_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for IamWorkforcePoolProvider {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IamWorkforcePoolProvider { }

impl terrars::ToListMappable for IamWorkforcePoolProvider {
    type O = terrars::ListRef<IamWorkforcePoolProviderRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IamWorkforcePoolProvider_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_iam_workforce_pool_provider".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIamWorkforcePoolProvider {
    pub tf_id: std::string::String,
    #[doc = "The location for the resource."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID for the provider, which becomes the final component of the resource name.\nThis value must be 4-32 characters, and may contain the characters [a-z0-9-].\nThe prefix 'gcp-' is reserved for use by Google, and may not be specified."]
    pub provider_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID to use for the pool, which becomes the final component of the resource name.\nThe IDs must be a globally unique string of 6 to 63 lowercase letters, digits, or hyphens.\nIt must start with a letter, and cannot have a trailing hyphen.\nThe prefix 'gcp-' is reserved for use by Google, and may not be specified."]
    pub workforce_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildIamWorkforcePoolProvider {
    pub fn build(self, stack: &mut terrars::Stack) -> IamWorkforcePoolProvider {
        let out = IamWorkforcePoolProvider(std::rc::Rc::new(IamWorkforcePoolProvider_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IamWorkforcePoolProviderData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                attribute_condition: core::default::Default::default(),
                attribute_mapping: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                detailed_audit_logging: core::default::Default::default(),
                disabled: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                provider_id: self.provider_id,
                scim_usage: core::default::Default::default(),
                workforce_pool_id: self.workforce_pool_id,
                extended_attributes_oauth2_client: core::default::Default::default(),
                extra_attributes_oauth2_client: core::default::Default::default(),
                oidc: core::default::Default::default(),
                saml: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IamWorkforcePoolProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkforcePoolProviderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IamWorkforcePoolProviderRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attribute_condition` after provisioning.\nA [Common Expression Language](https://github.com/google/cel-spec) expression, in\nplain text, to restrict what otherwise valid authentication credentials issued by the\nprovider should not be accepted.\n\nThe expression must output a boolean representing whether to allow the federation.\n\nThe following keywords may be referenced in the expressions:\n  * 'assertion': JSON representing the authentication credential issued by the provider.\n  * 'google': The Google attributes mapped from the assertion in the 'attribute_mappings'.\n    'google.profile_photo' and 'google.display_name' are not supported.\n  * 'attribute': The custom attributes mapped from the assertion in the 'attribute_mappings'.\n\nThe maximum length of the attribute condition expression is 4096 characters.\nIf unspecified, all valid authentication credentials will be accepted.\n\nThe following example shows how to only allow credentials with a mapped 'google.groups' value of 'admins':\n'''\n\"'admins' in google.groups\"\n'''"]
    pub fn attribute_condition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_condition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `attribute_mapping` after provisioning.\nMaps attributes from the authentication credentials issued by an external identity provider\nto Google Cloud attributes, such as 'subject' and 'segment'.\n\nEach key must be a string specifying the Google Cloud IAM attribute to map to.\n\nThe following keys are supported:\n  * 'google.subject': The principal IAM is authenticating. You can reference this value in IAM bindings.\n    This is also the subject that appears in Cloud Logging logs. This is a required field and\n    the mapped subject cannot exceed 127 bytes.\n  * 'google.groups': Groups the authenticating user belongs to. You can grant groups access to\n    resources using an IAM 'principalSet' binding; access applies to all members of the group.\n  * 'google.display_name': The name of the authenticated user. This is an optional field and\n    the mapped display name cannot exceed 100 bytes. If not set, 'google.subject' will be displayed instead.\n    This attribute cannot be referenced in IAM bindings.\n  * 'google.profile_photo': The URL that specifies the authenticated user's thumbnail photo.\n    This is an optional field. When set, the image will be visible as the user's profile picture.\n    If not set, a generic user icon will be displayed instead.\n    This attribute cannot be referenced in IAM bindings.\n\nYou can also provide custom attributes by specifying 'attribute.{custom_attribute}', where {custom_attribute}\nis the name of the custom attribute to be mapped. You can define a maximum of 50 custom attributes.\nThe maximum length of a mapped attribute key is 100 characters, and the key may only contain the characters [a-z0-9_].\n\nYou can reference these attributes in IAM policies to define fine-grained access for a workforce pool\nto Google Cloud resources. For example:\n  * 'google.subject':\n    'principal://iam.googleapis.com/locations/{location}/workforcePools/{pool}/subject/{value}'\n  * 'google.groups':\n    'principalSet://iam.googleapis.com/locations/{location}/workforcePools/{pool}/group/{value}'\n  * 'attribute.{custom_attribute}':\n    'principalSet://iam.googleapis.com/locations/{location}/workforcePools/{pool}/attribute.{custom_attribute}/{value}'\n\nEach value must be a [Common Expression Language](https://github.com/google/cel-spec)\nfunction that maps an identity provider credential to the normalized attribute specified\nby the corresponding map key.\n\nYou can use the 'assertion' keyword in the expression to access a JSON representation of\nthe authentication credential issued by the provider.\n\nThe maximum length of an attribute mapping expression is 2048 characters. When evaluated,\nthe total size of all mapped attributes must not exceed 8KB.\n\nFor OIDC providers, you must supply a custom mapping that includes the 'google.subject' attribute.\nFor example, the following maps the sub claim of the incoming credential to the 'subject' attribute\non a Google token:\n'''\n{\"google.subject\": \"assertion.sub\"}\n'''\n\nAn object containing a list of '\"key\": value' pairs.\nExample: '{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }'."]
    pub fn attribute_mapping(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.attribute_mapping", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA user-specified description of the provider. Cannot exceed 256 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `detailed_audit_logging` after provisioning.\nIf true, populates additional debug information in Cloud Audit Logs for this provider. Logged attribute mappings and values can be found in 'sts.googleapis.com' data access logs. Default value is false."]
    pub fn detailed_audit_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detailed_audit_logging", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the provider is disabled. You cannot use a disabled provider to exchange tokens.\nHowever, existing tokens still grant access."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA user-specified display name for the provider. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. The resource name of the provider.\nFormat: 'locations/{location}/workforcePools/{workforcePoolId}/providers/{providerId}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provider_id` after provisioning.\nThe ID for the provider, which becomes the final component of the resource name.\nThis value must be 4-32 characters, and may contain the characters [a-z0-9-].\nThe prefix 'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scim_usage` after provisioning.\nAgentspace only. Specifies whether the workforce identity pool\nprovider uses SCIM-managed groups instead of the 'google.groups'\nattribute mapping for authorization checks.\n\nThe 'scimUsage' and 'extendedAttributesOauth2Client' fields are\nmutually exclusive. A request that enables both fields on the same\nworkforce identity pool provider will produce an error.\n* SCIM_USAGE_UNSPECIFIED: Default behaviour\n* ENABLED_FOR_GROUPS: Use SCIM-managed groups instead of the 'google.groups'\n  attribute mapping for authorization checks Possible values: [\"SCIM_USAGE_UNSPECIFIED\", \"ENABLED_FOR_GROUPS\"]"]
    pub fn scim_usage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scim_usage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the provider.\n* STATE_UNSPECIFIED: State unspecified.\n* ACTIVE: The provider is active and may be used to validate authentication credentials.\n* DELETED: The provider is soft-deleted. Soft-deleted providers are permanently\n  deleted after approximately 30 days. You can restore a soft-deleted provider using\n  [providers.undelete](https://cloud.google.com/iam/docs/reference/rest/v1/locations.workforcePools.providers/undelete#google.iam.admin.v1.WorkforcePools.UndeleteWorkforcePoolProvider)."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workforce_pool_id` after provisioning.\nThe ID to use for the pool, which becomes the final component of the resource name.\nThe IDs must be a globally unique string of 6 to 63 lowercase letters, digits, or hyphens.\nIt must start with a letter, and cannot have a trailing hyphen.\nThe prefix 'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workforce_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workforce_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `extended_attributes_oauth2_client` after provisioning.\n"]
    pub fn extended_attributes_oauth2_client(
        &self,
    ) -> terrars::ListRef<IamWorkforcePoolProvider_ExtendedAttributesOauth2ClientRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.extended_attributes_oauth2_client", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `extra_attributes_oauth2_client` after provisioning.\n"]
    pub fn extra_attributes_oauth2_client(
        &self,
    ) -> terrars::ListRef<IamWorkforcePoolProvider_ExtraAttributesOauth2ClientRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.extra_attributes_oauth2_client", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> terrars::ListRef<IamWorkforcePoolProvider_OidcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> terrars::ListRef<IamWorkforcePoolProvider_SamlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkforcePoolProvider_TimeoutsRef {
        IamWorkforcePoolProvider_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct IamWorkforcePoolProviderDynamic {
    extended_attributes_oauth2_client: std
    ::option
    ::Option<terrars::DynamicBlock<IamWorkforcePoolProvider_ExtendedAttributesOauth2Client>>,
    extra_attributes_oauth2_client: std
    ::option
    ::Option<terrars::DynamicBlock<IamWorkforcePoolProvider_ExtraAttributesOauth2Client>>,
    oidc: std::option::Option<terrars::DynamicBlock<IamWorkforcePoolProvider_Oidc>>,
    saml: std::option::Option<terrars::DynamicBlock<IamWorkforcePoolProvider_Saml>>,
}

#[derive(Serialize)]
pub struct IamWorkforcePoolProvider_Oidc_ClientSecret_Value {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plain_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plain_text_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plain_text_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IamWorkforcePoolProvider_Oidc_ClientSecret_Value {
    #[doc = "Set the field `plain_text`.\nThe plain text of the client secret value."]
    pub fn set_plain_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.plain_text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `plain_text_wo`.\nThe plain text of the client secret value."]
    pub fn set_plain_text_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.plain_text_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `plain_text_wo_version`.\nTriggers update of 'plain_text_wo' write-only. Increment this value when an update to 'plain_text_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_plain_text_wo_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.plain_text_wo_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IamWorkforcePoolProvider_Oidc_ClientSecret_Value {
    type O = terrars::BlockAssignable<IamWorkforcePoolProvider_Oidc_ClientSecret_Value>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolProvider_Oidc_ClientSecret_Value {}

impl BuildIamWorkforcePoolProvider_Oidc_ClientSecret_Value {
    pub fn build(self) -> IamWorkforcePoolProvider_Oidc_ClientSecret_Value {
        IamWorkforcePoolProvider_Oidc_ClientSecret_Value {
            plain_text: core::default::Default::default(),
            plain_text_wo: core::default::Default::default(),
            plain_text_wo_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkforcePoolProvider_Oidc_ClientSecret_ValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkforcePoolProvider_Oidc_ClientSecret_ValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamWorkforcePoolProvider_Oidc_ClientSecret_ValueRef {
        IamWorkforcePoolProvider_Oidc_ClientSecret_ValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolProvider_Oidc_ClientSecret_ValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `plain_text` after provisioning.\nThe plain text of the client secret value."]
    pub fn plain_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plain_text", self.base))
    }

    #[doc =
        "Get a reference to the value of field `plain_text_wo` after provisioning.\nThe plain text of the client secret value."]
    pub fn plain_text_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plain_text_wo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `plain_text_wo_version` after provisioning.\nTriggers update of 'plain_text_wo' write-only. Increment this value when an update to 'plain_text_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn plain_text_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plain_text_wo_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `thumbprint` after provisioning.\nA thumbprint to represent the current client secret value."]
    pub fn thumbprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thumbprint", self.base))
    }
}

#[derive(Serialize, Default)]
struct IamWorkforcePoolProvider_Oidc_ClientSecretDynamic {
    value: std::option::Option<terrars::DynamicBlock<IamWorkforcePoolProvider_Oidc_ClientSecret_Value>>,
}

#[derive(Serialize)]
pub struct IamWorkforcePoolProvider_Oidc_ClientSecret {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<std::vec::Vec<IamWorkforcePoolProvider_Oidc_ClientSecret_Value>>,
    dynamic: IamWorkforcePoolProvider_Oidc_ClientSecretDynamic,
}

impl IamWorkforcePoolProvider_Oidc_ClientSecret {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamWorkforcePoolProvider_Oidc_ClientSecret_Value>>,
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

impl terrars::ToListMappable for IamWorkforcePoolProvider_Oidc_ClientSecret {
    type O = terrars::BlockAssignable<IamWorkforcePoolProvider_Oidc_ClientSecret>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolProvider_Oidc_ClientSecret {}

impl BuildIamWorkforcePoolProvider_Oidc_ClientSecret {
    pub fn build(self) -> IamWorkforcePoolProvider_Oidc_ClientSecret {
        IamWorkforcePoolProvider_Oidc_ClientSecret {
            value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkforcePoolProvider_Oidc_ClientSecretRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkforcePoolProvider_Oidc_ClientSecretRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamWorkforcePoolProvider_Oidc_ClientSecretRef {
        IamWorkforcePoolProvider_Oidc_ClientSecretRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolProvider_Oidc_ClientSecretRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::ListRef<IamWorkforcePoolProvider_Oidc_ClientSecret_ValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct IamWorkforcePoolProvider_ExtendedAttributesOauth2Client_QueryParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IamWorkforcePoolProvider_ExtendedAttributesOauth2Client_QueryParameters {
    #[doc =
        "Set the field `filter`.\nThe filter used to request specific records from IdP. In case of attributes type as AZURE_AD_GROUPS_ID, it represents the\nfilter used to request specific groups for users from IdP. By default, all of the groups associated with the user are fetched. The\ngroups should be security enabled. See https://learn.microsoft.com/en-us/graph/search-query-parameter for more details."]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IamWorkforcePoolProvider_ExtendedAttributesOauth2Client_QueryParameters {
    type O = terrars::BlockAssignable<IamWorkforcePoolProvider_ExtendedAttributesOauth2Client_QueryParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolProvider_ExtendedAttributesOauth2Client_QueryParameters {}

impl BuildIamWorkforcePoolProvider_ExtendedAttributesOauth2Client_QueryParameters {
    pub fn build(self) -> IamWorkforcePoolProvider_ExtendedAttributesOauth2Client_QueryParameters {
        IamWorkforcePoolProvider_ExtendedAttributesOauth2Client_QueryParameters {
            filter: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkforcePoolProvider_ExtendedAttributesOauth2Client_QueryParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkforcePoolProvider_ExtendedAttributesOauth2Client_QueryParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamWorkforcePoolProvider_ExtendedAttributesOauth2Client_QueryParametersRef {
        IamWorkforcePoolProvider_ExtendedAttributesOauth2Client_QueryParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolProvider_ExtendedAttributesOauth2Client_QueryParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nThe filter used to request specific records from IdP. In case of attributes type as AZURE_AD_GROUPS_ID, it represents the\nfilter used to request specific groups for users from IdP. By default, all of the groups associated with the user are fetched. The\ngroups should be security enabled. See https://learn.microsoft.com/en-us/graph/search-query-parameter for more details."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct IamWorkforcePoolProvider_ExtendedAttributesOauth2ClientDynamic {
    client_secret: std::option::Option<terrars::DynamicBlock<IamWorkforcePoolProvider_Oidc_ClientSecret>>,
    query_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<IamWorkforcePoolProvider_ExtendedAttributesOauth2Client_QueryParameters>>,
}

#[derive(Serialize)]
pub struct IamWorkforcePoolProvider_ExtendedAttributesOauth2Client {
    attributes_type: terrars::PrimField<std::string::String>,
    client_id: terrars::PrimField<std::string::String>,
    issuer_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<std::vec::Vec<IamWorkforcePoolProvider_Oidc_ClientSecret>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_parameters: std
    ::option
    ::Option<std::vec::Vec<IamWorkforcePoolProvider_ExtendedAttributesOauth2Client_QueryParameters>>,
    dynamic: IamWorkforcePoolProvider_ExtendedAttributesOauth2ClientDynamic,
}

impl IamWorkforcePoolProvider_ExtendedAttributesOauth2Client {
    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamWorkforcePoolProvider_Oidc_ClientSecret>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_secret = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_secret = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_parameters`.\n"]
    pub fn set_query_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<IamWorkforcePoolProvider_ExtendedAttributesOauth2Client_QueryParameters>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IamWorkforcePoolProvider_ExtendedAttributesOauth2Client {
    type O = terrars::BlockAssignable<IamWorkforcePoolProvider_ExtendedAttributesOauth2Client>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolProvider_ExtendedAttributesOauth2Client {
    #[doc =
        "Represents the IdP and type of claims that should be fetched.\n* AZURE_AD_GROUPS_ID:  Used to get the user's group claims from the Azure AD identity provider\nusing configuration provided in ExtendedAttributesOAuth2Client and 'id'\nproperty of the 'microsoft.graph.group' object is used for claim mapping. See\nhttps://learn.microsoft.com/en-us/graph/api/resources/group?view=graph-rest-1.0#properties\nfor more details on 'microsoft.graph.group' properties. The\ngroup IDs obtained from Azure AD are present in 'assertion.groups' for\nOIDC providers and 'assertion.attributes.groups' for SAML providers for\nattribute mapping. Possible values: [\"AZURE_AD_GROUPS_ID\"]"]
    pub attributes_type: terrars::PrimField<std::string::String>,
    #[doc =
        "The OAuth 2.0 client ID for retrieving extended attributes from the identity provider. Required to get the Access Token using client credentials grant flow."]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The OIDC identity provider's issuer URI. Must be a valid URI using the 'https' scheme. Required to get the OIDC discovery document."]
    pub issuer_uri: terrars::PrimField<std::string::String>,
}

impl BuildIamWorkforcePoolProvider_ExtendedAttributesOauth2Client {
    pub fn build(self) -> IamWorkforcePoolProvider_ExtendedAttributesOauth2Client {
        IamWorkforcePoolProvider_ExtendedAttributesOauth2Client {
            attributes_type: self.attributes_type,
            client_id: self.client_id,
            issuer_uri: self.issuer_uri,
            client_secret: core::default::Default::default(),
            query_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkforcePoolProvider_ExtendedAttributesOauth2ClientRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkforcePoolProvider_ExtendedAttributesOauth2ClientRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamWorkforcePoolProvider_ExtendedAttributesOauth2ClientRef {
        IamWorkforcePoolProvider_ExtendedAttributesOauth2ClientRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolProvider_ExtendedAttributesOauth2ClientRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attributes_type` after provisioning.\nRepresents the IdP and type of claims that should be fetched.\n* AZURE_AD_GROUPS_ID:  Used to get the user's group claims from the Azure AD identity provider\nusing configuration provided in ExtendedAttributesOAuth2Client and 'id'\nproperty of the 'microsoft.graph.group' object is used for claim mapping. See\nhttps://learn.microsoft.com/en-us/graph/api/resources/group?view=graph-rest-1.0#properties\nfor more details on 'microsoft.graph.group' properties. The\ngroup IDs obtained from Azure AD are present in 'assertion.groups' for\nOIDC providers and 'assertion.attributes.groups' for SAML providers for\nattribute mapping. Possible values: [\"AZURE_AD_GROUPS_ID\"]"]
    pub fn attributes_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attributes_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nThe OAuth 2.0 client ID for retrieving extended attributes from the identity provider. Required to get the Access Token using client credentials grant flow."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `issuer_uri` after provisioning.\nThe OIDC identity provider's issuer URI. Must be a valid URI using the 'https' scheme. Required to get the OIDC discovery document."]
    pub fn issuer_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::ListRef<IamWorkforcePoolProvider_Oidc_ClientSecretRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `query_parameters` after provisioning.\n"]
    pub fn query_parameters(
        &self,
    ) -> terrars::ListRef<IamWorkforcePoolProvider_ExtendedAttributesOauth2Client_QueryParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct IamWorkforcePoolProvider_ExtraAttributesOauth2Client_QueryParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IamWorkforcePoolProvider_ExtraAttributesOauth2Client_QueryParameters {
    #[doc =
        "Set the field `filter`.\nThe filter used to request specific records from IdP. In case of attributes type as AZURE_AD_GROUPS_MAIL,\nAZURE_AD_GROUPS_ID and AZURE_AD_GROUPS_DISPLAY_NAME, it represents the filter used to request specific\ngroups for users from IdP. By default, all of the groups associated with the user are fetched. The groups\nshould be security enabled. See https://learn.microsoft.com/en-us/graph/search-query-parameter for more\ndetails."]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IamWorkforcePoolProvider_ExtraAttributesOauth2Client_QueryParameters {
    type O = terrars::BlockAssignable<IamWorkforcePoolProvider_ExtraAttributesOauth2Client_QueryParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolProvider_ExtraAttributesOauth2Client_QueryParameters {}

impl BuildIamWorkforcePoolProvider_ExtraAttributesOauth2Client_QueryParameters {
    pub fn build(self) -> IamWorkforcePoolProvider_ExtraAttributesOauth2Client_QueryParameters {
        IamWorkforcePoolProvider_ExtraAttributesOauth2Client_QueryParameters {
            filter: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkforcePoolProvider_ExtraAttributesOauth2Client_QueryParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkforcePoolProvider_ExtraAttributesOauth2Client_QueryParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamWorkforcePoolProvider_ExtraAttributesOauth2Client_QueryParametersRef {
        IamWorkforcePoolProvider_ExtraAttributesOauth2Client_QueryParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolProvider_ExtraAttributesOauth2Client_QueryParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nThe filter used to request specific records from IdP. In case of attributes type as AZURE_AD_GROUPS_MAIL,\nAZURE_AD_GROUPS_ID and AZURE_AD_GROUPS_DISPLAY_NAME, it represents the filter used to request specific\ngroups for users from IdP. By default, all of the groups associated with the user are fetched. The groups\nshould be security enabled. See https://learn.microsoft.com/en-us/graph/search-query-parameter for more\ndetails."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct IamWorkforcePoolProvider_ExtraAttributesOauth2ClientDynamic {
    client_secret: std::option::Option<terrars::DynamicBlock<IamWorkforcePoolProvider_Oidc_ClientSecret>>,
    query_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<IamWorkforcePoolProvider_ExtraAttributesOauth2Client_QueryParameters>>,
}

#[derive(Serialize)]
pub struct IamWorkforcePoolProvider_ExtraAttributesOauth2Client {
    attributes_type: terrars::PrimField<std::string::String>,
    client_id: terrars::PrimField<std::string::String>,
    issuer_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<std::vec::Vec<IamWorkforcePoolProvider_Oidc_ClientSecret>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_parameters: std
    ::option
    ::Option<std::vec::Vec<IamWorkforcePoolProvider_ExtraAttributesOauth2Client_QueryParameters>>,
    dynamic: IamWorkforcePoolProvider_ExtraAttributesOauth2ClientDynamic,
}

impl IamWorkforcePoolProvider_ExtraAttributesOauth2Client {
    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamWorkforcePoolProvider_Oidc_ClientSecret>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_secret = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_secret = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_parameters`.\n"]
    pub fn set_query_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IamWorkforcePoolProvider_ExtraAttributesOauth2Client_QueryParameters>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IamWorkforcePoolProvider_ExtraAttributesOauth2Client {
    type O = terrars::BlockAssignable<IamWorkforcePoolProvider_ExtraAttributesOauth2Client>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolProvider_ExtraAttributesOauth2Client {
    #[doc =
        "Represents the IdP and type of claims that should be fetched.\n* AZURE_AD_GROUPS_MAIL: Used to get the user's group claims from the Azure AD identity provider using configuration provided\nin ExtraAttributesOAuth2Client and 'mail' property of the 'microsoft.graph.group' object is used for claim mapping.\nSee https://learn.microsoft.com/en-us/graph/api/resources/group?view=graph-rest-1.0#properties for more details on\n'microsoft.graph.group' properties. The attributes obtained from idntity provider are mapped to 'assertion.groups'.\n* AZURE_AD_GROUPS_ID:  Used to get the user's group claims from the Azure AD identity provider\nusing configuration provided in ExtraAttributesOAuth2Client and 'id'\nproperty of the 'microsoft.graph.group' object is used for claim mapping. See\nhttps://learn.microsoft.com/en-us/graph/api/resources/group?view=graph-rest-1.0#properties\nfor more details on 'microsoft.graph.group' properties. The\ngroup IDs obtained from Azure AD are present in 'assertion.groups' for\nOIDC providers and 'assertion.attributes.groups' for SAML providers for\nattribute mapping.\n* AZURE_AD_GROUPS_DISPLAY_NAME: Used to get the user's group claims from the Azure AD identity provider\nusing configuration provided in ExtraAttributesOAuth2Client and 'displayName' property\nof the 'microsoft.graph.group' object is used for claim mapping. See\nhttps://learn.microsoft.com/en-us/graph/api/resources/group?view=graph-rest-1.0#properties\nfor more details on 'microsoft.graph.group' properties. The\ngroup displayNames obtained from Azure AD are present in 'assertion.groups' for\nOIDC providers and 'assertion.attributes.groups' for SAML providers for\nattribute mapping. Possible values: [\"AZURE_AD_GROUPS_MAIL\", \"AZURE_AD_GROUPS_ID\", \"AZURE_AD_GROUPS_DISPLAY_NAME\"]"]
    pub attributes_type: terrars::PrimField<std::string::String>,
    #[doc =
        "The OAuth 2.0 client ID for retrieving extra attributes from the identity provider. Required to get the Access Token using client credentials grant flow."]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The OIDC identity provider's issuer URI. Must be a valid URI using the 'https' scheme. Required to get the OIDC discovery document."]
    pub issuer_uri: terrars::PrimField<std::string::String>,
}

impl BuildIamWorkforcePoolProvider_ExtraAttributesOauth2Client {
    pub fn build(self) -> IamWorkforcePoolProvider_ExtraAttributesOauth2Client {
        IamWorkforcePoolProvider_ExtraAttributesOauth2Client {
            attributes_type: self.attributes_type,
            client_id: self.client_id,
            issuer_uri: self.issuer_uri,
            client_secret: core::default::Default::default(),
            query_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkforcePoolProvider_ExtraAttributesOauth2ClientRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkforcePoolProvider_ExtraAttributesOauth2ClientRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamWorkforcePoolProvider_ExtraAttributesOauth2ClientRef {
        IamWorkforcePoolProvider_ExtraAttributesOauth2ClientRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolProvider_ExtraAttributesOauth2ClientRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attributes_type` after provisioning.\nRepresents the IdP and type of claims that should be fetched.\n* AZURE_AD_GROUPS_MAIL: Used to get the user's group claims from the Azure AD identity provider using configuration provided\nin ExtraAttributesOAuth2Client and 'mail' property of the 'microsoft.graph.group' object is used for claim mapping.\nSee https://learn.microsoft.com/en-us/graph/api/resources/group?view=graph-rest-1.0#properties for more details on\n'microsoft.graph.group' properties. The attributes obtained from idntity provider are mapped to 'assertion.groups'.\n* AZURE_AD_GROUPS_ID:  Used to get the user's group claims from the Azure AD identity provider\nusing configuration provided in ExtraAttributesOAuth2Client and 'id'\nproperty of the 'microsoft.graph.group' object is used for claim mapping. See\nhttps://learn.microsoft.com/en-us/graph/api/resources/group?view=graph-rest-1.0#properties\nfor more details on 'microsoft.graph.group' properties. The\ngroup IDs obtained from Azure AD are present in 'assertion.groups' for\nOIDC providers and 'assertion.attributes.groups' for SAML providers for\nattribute mapping.\n* AZURE_AD_GROUPS_DISPLAY_NAME: Used to get the user's group claims from the Azure AD identity provider\nusing configuration provided in ExtraAttributesOAuth2Client and 'displayName' property\nof the 'microsoft.graph.group' object is used for claim mapping. See\nhttps://learn.microsoft.com/en-us/graph/api/resources/group?view=graph-rest-1.0#properties\nfor more details on 'microsoft.graph.group' properties. The\ngroup displayNames obtained from Azure AD are present in 'assertion.groups' for\nOIDC providers and 'assertion.attributes.groups' for SAML providers for\nattribute mapping. Possible values: [\"AZURE_AD_GROUPS_MAIL\", \"AZURE_AD_GROUPS_ID\", \"AZURE_AD_GROUPS_DISPLAY_NAME\"]"]
    pub fn attributes_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attributes_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nThe OAuth 2.0 client ID for retrieving extra attributes from the identity provider. Required to get the Access Token using client credentials grant flow."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `issuer_uri` after provisioning.\nThe OIDC identity provider's issuer URI. Must be a valid URI using the 'https' scheme. Required to get the OIDC discovery document."]
    pub fn issuer_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::ListRef<IamWorkforcePoolProvider_Oidc_ClientSecretRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `query_parameters` after provisioning.\n"]
    pub fn query_parameters(
        &self,
    ) -> terrars::ListRef<IamWorkforcePoolProvider_ExtraAttributesOauth2Client_QueryParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct IamWorkforcePoolProvider_Oidc_WebSsoConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    assertion_claims_behavior: terrars::PrimField<std::string::String>,
    response_type: terrars::PrimField<std::string::String>,
}

impl IamWorkforcePoolProvider_Oidc_WebSsoConfig {
    #[doc =
        "Set the field `additional_scopes`.\nAdditional scopes to request for in the OIDC authentication request on top of scopes requested by default. By default, the 'openid', 'profile' and 'email' scopes that are supported by the identity provider are requested.\nEach additional scope may be at most 256 characters. A maximum of 10 additional scopes may be configured."]
    pub fn set_additional_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.additional_scopes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IamWorkforcePoolProvider_Oidc_WebSsoConfig {
    type O = terrars::BlockAssignable<IamWorkforcePoolProvider_Oidc_WebSsoConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolProvider_Oidc_WebSsoConfig {
    #[doc =
        "The behavior for how OIDC Claims are included in the 'assertion' object used for attribute mapping and attribute condition.\n* MERGE_USER_INFO_OVER_ID_TOKEN_CLAIMS: Merge the UserInfo Endpoint Claims with ID Token Claims, preferring UserInfo Claim Values for the same Claim Name. This option is available only for the Authorization Code Flow.\n* ONLY_ID_TOKEN_CLAIMS: Only include ID Token Claims. Possible values: [\"MERGE_USER_INFO_OVER_ID_TOKEN_CLAIMS\", \"ONLY_ID_TOKEN_CLAIMS\"]"]
    pub assertion_claims_behavior: terrars::PrimField<std::string::String>,
    #[doc =
        "The Response Type to request for in the OIDC Authorization Request for web sign-in.\n\nThe 'CODE' Response Type is recommended to avoid the Implicit Flow, for security reasons.\n* CODE: The 'response_type=code' selection uses the Authorization Code Flow for web sign-in. Requires a configured client secret.\n* ID_TOKEN: The 'response_type=id_token' selection uses the Implicit Flow for web sign-in. Possible values: [\"CODE\", \"ID_TOKEN\"]"]
    pub response_type: terrars::PrimField<std::string::String>,
}

impl BuildIamWorkforcePoolProvider_Oidc_WebSsoConfig {
    pub fn build(self) -> IamWorkforcePoolProvider_Oidc_WebSsoConfig {
        IamWorkforcePoolProvider_Oidc_WebSsoConfig {
            additional_scopes: core::default::Default::default(),
            assertion_claims_behavior: self.assertion_claims_behavior,
            response_type: self.response_type,
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkforcePoolProvider_Oidc_WebSsoConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkforcePoolProvider_Oidc_WebSsoConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamWorkforcePoolProvider_Oidc_WebSsoConfigRef {
        IamWorkforcePoolProvider_Oidc_WebSsoConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolProvider_Oidc_WebSsoConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additional_scopes` after provisioning.\nAdditional scopes to request for in the OIDC authentication request on top of scopes requested by default. By default, the 'openid', 'profile' and 'email' scopes that are supported by the identity provider are requested.\nEach additional scope may be at most 256 characters. A maximum of 10 additional scopes may be configured."]
    pub fn additional_scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `assertion_claims_behavior` after provisioning.\nThe behavior for how OIDC Claims are included in the 'assertion' object used for attribute mapping and attribute condition.\n* MERGE_USER_INFO_OVER_ID_TOKEN_CLAIMS: Merge the UserInfo Endpoint Claims with ID Token Claims, preferring UserInfo Claim Values for the same Claim Name. This option is available only for the Authorization Code Flow.\n* ONLY_ID_TOKEN_CLAIMS: Only include ID Token Claims. Possible values: [\"MERGE_USER_INFO_OVER_ID_TOKEN_CLAIMS\", \"ONLY_ID_TOKEN_CLAIMS\"]"]
    pub fn assertion_claims_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.assertion_claims_behavior", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response_type` after provisioning.\nThe Response Type to request for in the OIDC Authorization Request for web sign-in.\n\nThe 'CODE' Response Type is recommended to avoid the Implicit Flow, for security reasons.\n* CODE: The 'response_type=code' selection uses the Authorization Code Flow for web sign-in. Requires a configured client secret.\n* ID_TOKEN: The 'response_type=id_token' selection uses the Implicit Flow for web sign-in. Possible values: [\"CODE\", \"ID_TOKEN\"]"]
    pub fn response_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct IamWorkforcePoolProvider_OidcDynamic {
    client_secret: std::option::Option<terrars::DynamicBlock<IamWorkforcePoolProvider_Oidc_ClientSecret>>,
    web_sso_config: std::option::Option<terrars::DynamicBlock<IamWorkforcePoolProvider_Oidc_WebSsoConfig>>,
}

#[derive(Serialize)]
pub struct IamWorkforcePoolProvider_Oidc {
    client_id: terrars::PrimField<std::string::String>,
    issuer_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwks_json: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<std::vec::Vec<IamWorkforcePoolProvider_Oidc_ClientSecret>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_sso_config: std::option::Option<std::vec::Vec<IamWorkforcePoolProvider_Oidc_WebSsoConfig>>,
    dynamic: IamWorkforcePoolProvider_OidcDynamic,
}

impl IamWorkforcePoolProvider_Oidc {
    #[doc =
        "Set the field `jwks_json`.\nOIDC JWKs in JSON String format. For details on definition of a\nJWK, see https:tools.ietf.org/html/rfc7517. If not set, then we\nuse the 'jwks_uri' from the discovery document fetched from the\n.well-known path for the 'issuer_uri'. Currently, RSA and EC asymmetric\nkeys are supported. The JWK must use following format and include only\nthe following fields:\n'''\n{\n  \"keys\": [\n    {\n          \"kty\": \"RSA/EC\",\n          \"alg\": \"<algorithm>\",\n          \"use\": \"sig\",\n          \"kid\": \"<key-id>\",\n          \"n\": \"\",\n          \"e\": \"\",\n          \"x\": \"\",\n          \"y\": \"\",\n          \"crv\": \"\"\n    }\n  ]\n}\n'''"]
    pub fn set_jwks_json(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.jwks_json = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamWorkforcePoolProvider_Oidc_ClientSecret>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_secret = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_secret = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `web_sso_config`.\n"]
    pub fn set_web_sso_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamWorkforcePoolProvider_Oidc_WebSsoConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.web_sso_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.web_sso_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IamWorkforcePoolProvider_Oidc {
    type O = terrars::BlockAssignable<IamWorkforcePoolProvider_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolProvider_Oidc {
    #[doc = "The client ID. Must match the audience claim of the JWT issued by the identity provider."]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = "The OIDC issuer URI. Must be a valid URI using the 'https' scheme."]
    pub issuer_uri: terrars::PrimField<std::string::String>,
}

impl BuildIamWorkforcePoolProvider_Oidc {
    pub fn build(self) -> IamWorkforcePoolProvider_Oidc {
        IamWorkforcePoolProvider_Oidc {
            client_id: self.client_id,
            issuer_uri: self.issuer_uri,
            jwks_json: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            web_sso_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkforcePoolProvider_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkforcePoolProvider_OidcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamWorkforcePoolProvider_OidcRef {
        IamWorkforcePoolProvider_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolProvider_OidcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nThe client ID. Must match the audience claim of the JWT issued by the identity provider."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `issuer_uri` after provisioning.\nThe OIDC issuer URI. Must be a valid URI using the 'https' scheme."]
    pub fn issuer_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jwks_json` after provisioning.\nOIDC JWKs in JSON String format. For details on definition of a\nJWK, see https:tools.ietf.org/html/rfc7517. If not set, then we\nuse the 'jwks_uri' from the discovery document fetched from the\n.well-known path for the 'issuer_uri'. Currently, RSA and EC asymmetric\nkeys are supported. The JWK must use following format and include only\nthe following fields:\n'''\n{\n  \"keys\": [\n    {\n          \"kty\": \"RSA/EC\",\n          \"alg\": \"<algorithm>\",\n          \"use\": \"sig\",\n          \"kid\": \"<key-id>\",\n          \"n\": \"\",\n          \"e\": \"\",\n          \"x\": \"\",\n          \"y\": \"\",\n          \"crv\": \"\"\n    }\n  ]\n}\n'''"]
    pub fn jwks_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwks_json", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::ListRef<IamWorkforcePoolProvider_Oidc_ClientSecretRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `web_sso_config` after provisioning.\n"]
    pub fn web_sso_config(&self) -> terrars::ListRef<IamWorkforcePoolProvider_Oidc_WebSsoConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_sso_config", self.base))
    }
}

#[derive(Serialize)]
pub struct IamWorkforcePoolProvider_Saml {
    idp_metadata_xml: terrars::PrimField<std::string::String>,
}

impl IamWorkforcePoolProvider_Saml { }

impl terrars::ToListMappable for IamWorkforcePoolProvider_Saml {
    type O = terrars::BlockAssignable<IamWorkforcePoolProvider_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolProvider_Saml {
    #[doc =
        "SAML Identity provider configuration metadata xml doc.\nThe xml document should comply with [SAML 2.0 specification](https://docs.oasis-open.org/security/saml/v2.0/saml-metadata-2.0-os.pdf).\nThe max size of the acceptable xml document will be bounded to 128k characters.\n\nThe metadata xml document should satisfy the following constraints:\n1) Must contain an Identity Provider Entity ID.\n2) Must contain at least one non-expired signing key certificate.\n3) For each signing key:\n  a) Valid from should be no more than 7 days from now.\n  b) Valid to should be no more than 10 years in the future.\n4) Up to 3 IdP signing keys are allowed in the metadata xml.\n\nWhen updating the provider's metadata xml, at least one non-expired signing key\nmust overlap with the existing metadata. This requirement is skipped if there are\nno non-expired signing keys present in the existing metadata."]
    pub idp_metadata_xml: terrars::PrimField<std::string::String>,
}

impl BuildIamWorkforcePoolProvider_Saml {
    pub fn build(self) -> IamWorkforcePoolProvider_Saml {
        IamWorkforcePoolProvider_Saml { idp_metadata_xml: self.idp_metadata_xml }
    }
}

#[allow(dead_code)]
pub struct IamWorkforcePoolProvider_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkforcePoolProvider_SamlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamWorkforcePoolProvider_SamlRef {
        IamWorkforcePoolProvider_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolProvider_SamlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `idp_metadata_xml` after provisioning.\nSAML Identity provider configuration metadata xml doc.\nThe xml document should comply with [SAML 2.0 specification](https://docs.oasis-open.org/security/saml/v2.0/saml-metadata-2.0-os.pdf).\nThe max size of the acceptable xml document will be bounded to 128k characters.\n\nThe metadata xml document should satisfy the following constraints:\n1) Must contain an Identity Provider Entity ID.\n2) Must contain at least one non-expired signing key certificate.\n3) For each signing key:\n  a) Valid from should be no more than 7 days from now.\n  b) Valid to should be no more than 10 years in the future.\n4) Up to 3 IdP signing keys are allowed in the metadata xml.\n\nWhen updating the provider's metadata xml, at least one non-expired signing key\nmust overlap with the existing metadata. This requirement is skipped if there are\nno non-expired signing keys present in the existing metadata."]
    pub fn idp_metadata_xml(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idp_metadata_xml", self.base))
    }
}

#[derive(Serialize)]
pub struct IamWorkforcePoolProvider_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IamWorkforcePoolProvider_Timeouts {
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

impl terrars::ToListMappable for IamWorkforcePoolProvider_Timeouts {
    type O = terrars::BlockAssignable<IamWorkforcePoolProvider_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolProvider_Timeouts {}

impl BuildIamWorkforcePoolProvider_Timeouts {
    pub fn build(self) -> IamWorkforcePoolProvider_Timeouts {
        IamWorkforcePoolProvider_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkforcePoolProvider_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkforcePoolProvider_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamWorkforcePoolProvider_TimeoutsRef {
        IamWorkforcePoolProvider_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolProvider_TimeoutsRef {
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
