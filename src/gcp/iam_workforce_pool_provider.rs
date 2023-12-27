use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IamWorkforcePoolProviderData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_condition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_mapping: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    provider_id: PrimField<String>,
    workforce_pool_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oidc: Option<Vec<IamWorkforcePoolProviderOidcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saml: Option<Vec<IamWorkforcePoolProviderSamlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IamWorkforcePoolProviderTimeoutsEl>,
    dynamic: IamWorkforcePoolProviderDynamic,
}

struct IamWorkforcePoolProvider_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IamWorkforcePoolProviderData>,
}

#[derive(Clone)]
pub struct IamWorkforcePoolProvider(Rc<IamWorkforcePoolProvider_>);

impl IamWorkforcePoolProvider {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `attribute_condition`.\nA [Common Expression Language](https://opensource.google/projects/cel) expression, in\nplain text, to restrict what otherwise valid authentication credentials issued by the\nprovider should not be accepted.\n\nThe expression must output a boolean representing whether to allow the federation.\n\nThe following keywords may be referenced in the expressions:\n  * 'assertion': JSON representing the authentication credential issued by the provider.\n  * 'google': The Google attributes mapped from the assertion in the 'attribute_mappings'.\n    'google.profile_photo' and 'google.display_name' are not supported.\n  * 'attribute': The custom attributes mapped from the assertion in the 'attribute_mappings'.\n\nThe maximum length of the attribute condition expression is 4096 characters.\nIf unspecified, all valid authentication credentials will be accepted.\n\nThe following example shows how to only allow credentials with a mapped 'google.groups' value of 'admins':\n'''\n\"'admins' in google.groups\"\n'''"]
    pub fn set_attribute_condition(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().attribute_condition = Some(v.into());
        self
    }

    #[doc= "Set the field `attribute_mapping`.\nMaps attributes from the authentication credentials issued by an external identity provider\nto Google Cloud attributes, such as 'subject' and 'segment'.\n\nEach key must be a string specifying the Google Cloud IAM attribute to map to.\n\nThe following keys are supported:\n  * 'google.subject': The principal IAM is authenticating. You can reference this value in IAM bindings.\n    This is also the subject that appears in Cloud Logging logs. This is a required field and\n    the mapped subject cannot exceed 127 bytes.\n  * 'google.groups': Groups the authenticating user belongs to. You can grant groups access to\n    resources using an IAM 'principalSet' binding; access applies to all members of the group.\n  * 'google.display_name': The name of the authenticated user. This is an optional field and\n    the mapped display name cannot exceed 100 bytes. If not set, 'google.subject' will be displayed instead.\n    This attribute cannot be referenced in IAM bindings.\n  * 'google.profile_photo': The URL that specifies the authenticated user's thumbnail photo.\n    This is an optional field. When set, the image will be visible as the user's profile picture.\n    If not set, a generic user icon will be displayed instead.\n    This attribute cannot be referenced in IAM bindings.\n\nYou can also provide custom attributes by specifying 'attribute.{custom_attribute}', where {custom_attribute}\nis the name of the custom attribute to be mapped. You can define a maximum of 50 custom attributes.\nThe maximum length of a mapped attribute key is 100 characters, and the key may only contain the characters [a-z0-9_].\n\nYou can reference these attributes in IAM policies to define fine-grained access for a workforce pool\nto Google Cloud resources. For example:\n  * 'google.subject':\n    'principal://iam.googleapis.com/locations/{location}/workforcePools/{pool}/subject/{value}'\n  * 'google.groups':\n    'principalSet://iam.googleapis.com/locations/{location}/workforcePools/{pool}/group/{value}'\n  * 'attribute.{custom_attribute}':\n    'principalSet://iam.googleapis.com/locations/{location}/workforcePools/{pool}/attribute.{custom_attribute}/{value}'\n\nEach value must be a [Common Expression Language](https://opensource.google/projects/cel)\nfunction that maps an identity provider credential to the normalized attribute specified\nby the corresponding map key.\n\nYou can use the 'assertion' keyword in the expression to access a JSON representation of\nthe authentication credential issued by the provider.\n\nThe maximum length of an attribute mapping expression is 2048 characters. When evaluated,\nthe total size of all mapped attributes must not exceed 8KB.\n\nFor OIDC providers, you must supply a custom mapping that includes the 'google.subject' attribute.\nFor example, the following maps the sub claim of the incoming credential to the 'subject' attribute\non a Google token:\n'''\n{\"google.subject\": \"assertion.sub\"}\n'''\n\nAn object containing a list of '\"key\": value' pairs.\nExample: '{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }'."]
    pub fn set_attribute_mapping(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().attribute_mapping = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nA user-specified description of the provider. Cannot exceed 256 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `disabled`.\nWhether the provider is disabled. You cannot use a disabled provider to exchange tokens.\nHowever, existing tokens still grant access."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }

    #[doc= "Set the field `display_name`.\nA user-specified display name for the provider. Cannot exceed 32 characters."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `oidc`.\n"]
    pub fn set_oidc(self, v: impl Into<BlockAssignable<IamWorkforcePoolProviderOidcEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oidc = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oidc = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `saml`.\n"]
    pub fn set_saml(self, v: impl Into<BlockAssignable<IamWorkforcePoolProviderSamlEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().saml = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.saml = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IamWorkforcePoolProviderTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `attribute_condition` after provisioning.\nA [Common Expression Language](https://opensource.google/projects/cel) expression, in\nplain text, to restrict what otherwise valid authentication credentials issued by the\nprovider should not be accepted.\n\nThe expression must output a boolean representing whether to allow the federation.\n\nThe following keywords may be referenced in the expressions:\n  * 'assertion': JSON representing the authentication credential issued by the provider.\n  * 'google': The Google attributes mapped from the assertion in the 'attribute_mappings'.\n    'google.profile_photo' and 'google.display_name' are not supported.\n  * 'attribute': The custom attributes mapped from the assertion in the 'attribute_mappings'.\n\nThe maximum length of the attribute condition expression is 4096 characters.\nIf unspecified, all valid authentication credentials will be accepted.\n\nThe following example shows how to only allow credentials with a mapped 'google.groups' value of 'admins':\n'''\n\"'admins' in google.groups\"\n'''"]
    pub fn attribute_condition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_condition", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `attribute_mapping` after provisioning.\nMaps attributes from the authentication credentials issued by an external identity provider\nto Google Cloud attributes, such as 'subject' and 'segment'.\n\nEach key must be a string specifying the Google Cloud IAM attribute to map to.\n\nThe following keys are supported:\n  * 'google.subject': The principal IAM is authenticating. You can reference this value in IAM bindings.\n    This is also the subject that appears in Cloud Logging logs. This is a required field and\n    the mapped subject cannot exceed 127 bytes.\n  * 'google.groups': Groups the authenticating user belongs to. You can grant groups access to\n    resources using an IAM 'principalSet' binding; access applies to all members of the group.\n  * 'google.display_name': The name of the authenticated user. This is an optional field and\n    the mapped display name cannot exceed 100 bytes. If not set, 'google.subject' will be displayed instead.\n    This attribute cannot be referenced in IAM bindings.\n  * 'google.profile_photo': The URL that specifies the authenticated user's thumbnail photo.\n    This is an optional field. When set, the image will be visible as the user's profile picture.\n    If not set, a generic user icon will be displayed instead.\n    This attribute cannot be referenced in IAM bindings.\n\nYou can also provide custom attributes by specifying 'attribute.{custom_attribute}', where {custom_attribute}\nis the name of the custom attribute to be mapped. You can define a maximum of 50 custom attributes.\nThe maximum length of a mapped attribute key is 100 characters, and the key may only contain the characters [a-z0-9_].\n\nYou can reference these attributes in IAM policies to define fine-grained access for a workforce pool\nto Google Cloud resources. For example:\n  * 'google.subject':\n    'principal://iam.googleapis.com/locations/{location}/workforcePools/{pool}/subject/{value}'\n  * 'google.groups':\n    'principalSet://iam.googleapis.com/locations/{location}/workforcePools/{pool}/group/{value}'\n  * 'attribute.{custom_attribute}':\n    'principalSet://iam.googleapis.com/locations/{location}/workforcePools/{pool}/attribute.{custom_attribute}/{value}'\n\nEach value must be a [Common Expression Language](https://opensource.google/projects/cel)\nfunction that maps an identity provider credential to the normalized attribute specified\nby the corresponding map key.\n\nYou can use the 'assertion' keyword in the expression to access a JSON representation of\nthe authentication credential issued by the provider.\n\nThe maximum length of an attribute mapping expression is 2048 characters. When evaluated,\nthe total size of all mapped attributes must not exceed 8KB.\n\nFor OIDC providers, you must supply a custom mapping that includes the 'google.subject' attribute.\nFor example, the following maps the sub claim of the incoming credential to the 'subject' attribute\non a Google token:\n'''\n{\"google.subject\": \"assertion.sub\"}\n'''\n\nAn object containing a list of '\"key\": value' pairs.\nExample: '{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }'."]
    pub fn attribute_mapping(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.attribute_mapping", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA user-specified description of the provider. Cannot exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\nWhether the provider is disabled. You cannot use a disabled provider to exchange tokens.\nHowever, existing tokens still grant access."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nA user-specified display name for the provider. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location for the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nOutput only. The resource name of the provider.\nFormat: 'locations/{location}/workforcePools/{workforcePoolId}/providers/{providerId}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `provider_id` after provisioning.\nThe ID for the provider, which becomes the final component of the resource name.\nThis value must be 4-32 characters, and may contain the characters [a-z0-9-].\nThe prefix 'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nThe current state of the provider.\n* STATE_UNSPECIFIED: State unspecified.\n* ACTIVE: The provider is active and may be used to validate authentication credentials.\n* DELETED: The provider is soft-deleted. Soft-deleted providers are permanently\n  deleted after approximately 30 days. You can restore a soft-deleted provider using\n  [providers.undelete](https://cloud.google.com/iam/docs/reference/rest/v1/locations.workforcePools.providers/undelete#google.iam.admin.v1.WorkforcePools.UndeleteWorkforcePoolProvider)."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `workforce_pool_id` after provisioning.\nThe ID to use for the pool, which becomes the final component of the resource name.\nThe IDs must be a globally unique string of 6 to 63 lowercase letters, digits, or hyphens.\nIt must start with a letter, and cannot have a trailing hyphen.\nThe prefix 'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workforce_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workforce_pool_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> ListRef<IamWorkforcePoolProviderOidcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oidc", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> ListRef<IamWorkforcePoolProviderSamlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.saml", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkforcePoolProviderTimeoutsElRef {
        IamWorkforcePoolProviderTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for IamWorkforcePoolProvider {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for IamWorkforcePoolProvider { }

impl ToListMappable for IamWorkforcePoolProvider {
    type O = ListRef<IamWorkforcePoolProviderRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for IamWorkforcePoolProvider_ {
    fn extract_resource_type(&self) -> String {
        "google_iam_workforce_pool_provider".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIamWorkforcePoolProvider {
    pub tf_id: String,
    #[doc= "The location for the resource."]
    pub location: PrimField<String>,
    #[doc= "The ID for the provider, which becomes the final component of the resource name.\nThis value must be 4-32 characters, and may contain the characters [a-z0-9-].\nThe prefix 'gcp-' is reserved for use by Google, and may not be specified."]
    pub provider_id: PrimField<String>,
    #[doc= "The ID to use for the pool, which becomes the final component of the resource name.\nThe IDs must be a globally unique string of 6 to 63 lowercase letters, digits, or hyphens.\nIt must start with a letter, and cannot have a trailing hyphen.\nThe prefix 'gcp-' is reserved for use by Google, and may not be specified."]
    pub workforce_pool_id: PrimField<String>,
}

impl BuildIamWorkforcePoolProvider {
    pub fn build(self, stack: &mut Stack) -> IamWorkforcePoolProvider {
        let out = IamWorkforcePoolProvider(Rc::new(IamWorkforcePoolProvider_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IamWorkforcePoolProviderData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                attribute_condition: core::default::Default::default(),
                attribute_mapping: core::default::Default::default(),
                description: core::default::Default::default(),
                disabled: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                provider_id: self.provider_id,
                workforce_pool_id: self.workforce_pool_id,
                oidc: core::default::Default::default(),
                saml: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IamWorkforcePoolProviderRef {
    shared: StackShared,
    base: String,
}

impl Ref for IamWorkforcePoolProviderRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IamWorkforcePoolProviderRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `attribute_condition` after provisioning.\nA [Common Expression Language](https://opensource.google/projects/cel) expression, in\nplain text, to restrict what otherwise valid authentication credentials issued by the\nprovider should not be accepted.\n\nThe expression must output a boolean representing whether to allow the federation.\n\nThe following keywords may be referenced in the expressions:\n  * 'assertion': JSON representing the authentication credential issued by the provider.\n  * 'google': The Google attributes mapped from the assertion in the 'attribute_mappings'.\n    'google.profile_photo' and 'google.display_name' are not supported.\n  * 'attribute': The custom attributes mapped from the assertion in the 'attribute_mappings'.\n\nThe maximum length of the attribute condition expression is 4096 characters.\nIf unspecified, all valid authentication credentials will be accepted.\n\nThe following example shows how to only allow credentials with a mapped 'google.groups' value of 'admins':\n'''\n\"'admins' in google.groups\"\n'''"]
    pub fn attribute_condition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_condition", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `attribute_mapping` after provisioning.\nMaps attributes from the authentication credentials issued by an external identity provider\nto Google Cloud attributes, such as 'subject' and 'segment'.\n\nEach key must be a string specifying the Google Cloud IAM attribute to map to.\n\nThe following keys are supported:\n  * 'google.subject': The principal IAM is authenticating. You can reference this value in IAM bindings.\n    This is also the subject that appears in Cloud Logging logs. This is a required field and\n    the mapped subject cannot exceed 127 bytes.\n  * 'google.groups': Groups the authenticating user belongs to. You can grant groups access to\n    resources using an IAM 'principalSet' binding; access applies to all members of the group.\n  * 'google.display_name': The name of the authenticated user. This is an optional field and\n    the mapped display name cannot exceed 100 bytes. If not set, 'google.subject' will be displayed instead.\n    This attribute cannot be referenced in IAM bindings.\n  * 'google.profile_photo': The URL that specifies the authenticated user's thumbnail photo.\n    This is an optional field. When set, the image will be visible as the user's profile picture.\n    If not set, a generic user icon will be displayed instead.\n    This attribute cannot be referenced in IAM bindings.\n\nYou can also provide custom attributes by specifying 'attribute.{custom_attribute}', where {custom_attribute}\nis the name of the custom attribute to be mapped. You can define a maximum of 50 custom attributes.\nThe maximum length of a mapped attribute key is 100 characters, and the key may only contain the characters [a-z0-9_].\n\nYou can reference these attributes in IAM policies to define fine-grained access for a workforce pool\nto Google Cloud resources. For example:\n  * 'google.subject':\n    'principal://iam.googleapis.com/locations/{location}/workforcePools/{pool}/subject/{value}'\n  * 'google.groups':\n    'principalSet://iam.googleapis.com/locations/{location}/workforcePools/{pool}/group/{value}'\n  * 'attribute.{custom_attribute}':\n    'principalSet://iam.googleapis.com/locations/{location}/workforcePools/{pool}/attribute.{custom_attribute}/{value}'\n\nEach value must be a [Common Expression Language](https://opensource.google/projects/cel)\nfunction that maps an identity provider credential to the normalized attribute specified\nby the corresponding map key.\n\nYou can use the 'assertion' keyword in the expression to access a JSON representation of\nthe authentication credential issued by the provider.\n\nThe maximum length of an attribute mapping expression is 2048 characters. When evaluated,\nthe total size of all mapped attributes must not exceed 8KB.\n\nFor OIDC providers, you must supply a custom mapping that includes the 'google.subject' attribute.\nFor example, the following maps the sub claim of the incoming credential to the 'subject' attribute\non a Google token:\n'''\n{\"google.subject\": \"assertion.sub\"}\n'''\n\nAn object containing a list of '\"key\": value' pairs.\nExample: '{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }'."]
    pub fn attribute_mapping(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.attribute_mapping", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA user-specified description of the provider. Cannot exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\nWhether the provider is disabled. You cannot use a disabled provider to exchange tokens.\nHowever, existing tokens still grant access."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nA user-specified display name for the provider. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location for the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nOutput only. The resource name of the provider.\nFormat: 'locations/{location}/workforcePools/{workforcePoolId}/providers/{providerId}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `provider_id` after provisioning.\nThe ID for the provider, which becomes the final component of the resource name.\nThis value must be 4-32 characters, and may contain the characters [a-z0-9-].\nThe prefix 'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nThe current state of the provider.\n* STATE_UNSPECIFIED: State unspecified.\n* ACTIVE: The provider is active and may be used to validate authentication credentials.\n* DELETED: The provider is soft-deleted. Soft-deleted providers are permanently\n  deleted after approximately 30 days. You can restore a soft-deleted provider using\n  [providers.undelete](https://cloud.google.com/iam/docs/reference/rest/v1/locations.workforcePools.providers/undelete#google.iam.admin.v1.WorkforcePools.UndeleteWorkforcePoolProvider)."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `workforce_pool_id` after provisioning.\nThe ID to use for the pool, which becomes the final component of the resource name.\nThe IDs must be a globally unique string of 6 to 63 lowercase letters, digits, or hyphens.\nIt must start with a letter, and cannot have a trailing hyphen.\nThe prefix 'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workforce_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workforce_pool_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> ListRef<IamWorkforcePoolProviderOidcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oidc", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> ListRef<IamWorkforcePoolProviderSamlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.saml", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkforcePoolProviderTimeoutsElRef {
        IamWorkforcePoolProviderTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct IamWorkforcePoolProviderOidcElClientSecretElValueEl {
    plain_text: PrimField<String>,
}

impl IamWorkforcePoolProviderOidcElClientSecretElValueEl { }

impl ToListMappable for IamWorkforcePoolProviderOidcElClientSecretElValueEl {
    type O = BlockAssignable<IamWorkforcePoolProviderOidcElClientSecretElValueEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolProviderOidcElClientSecretElValueEl {
    #[doc= "The plain text of the client secret value."]
    pub plain_text: PrimField<String>,
}

impl BuildIamWorkforcePoolProviderOidcElClientSecretElValueEl {
    pub fn build(self) -> IamWorkforcePoolProviderOidcElClientSecretElValueEl {
        IamWorkforcePoolProviderOidcElClientSecretElValueEl { plain_text: self.plain_text }
    }
}

pub struct IamWorkforcePoolProviderOidcElClientSecretElValueElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IamWorkforcePoolProviderOidcElClientSecretElValueElRef {
    fn new(shared: StackShared, base: String) -> IamWorkforcePoolProviderOidcElClientSecretElValueElRef {
        IamWorkforcePoolProviderOidcElClientSecretElValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolProviderOidcElClientSecretElValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `plain_text` after provisioning.\nThe plain text of the client secret value."]
    pub fn plain_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plain_text", self.base))
    }

    #[doc= "Get a reference to the value of field `thumbprint` after provisioning.\nA thumbprint to represent the current client secret value."]
    pub fn thumbprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.thumbprint", self.base))
    }
}

#[derive(Serialize, Default)]
struct IamWorkforcePoolProviderOidcElClientSecretElDynamic {
    value: Option<DynamicBlock<IamWorkforcePoolProviderOidcElClientSecretElValueEl>>,
}

#[derive(Serialize)]
pub struct IamWorkforcePoolProviderOidcElClientSecretEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Vec<IamWorkforcePoolProviderOidcElClientSecretElValueEl>>,
    dynamic: IamWorkforcePoolProviderOidcElClientSecretElDynamic,
}

impl IamWorkforcePoolProviderOidcElClientSecretEl {
    #[doc= "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v: impl Into<BlockAssignable<IamWorkforcePoolProviderOidcElClientSecretElValueEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.value = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.value = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for IamWorkforcePoolProviderOidcElClientSecretEl {
    type O = BlockAssignable<IamWorkforcePoolProviderOidcElClientSecretEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolProviderOidcElClientSecretEl {}

impl BuildIamWorkforcePoolProviderOidcElClientSecretEl {
    pub fn build(self) -> IamWorkforcePoolProviderOidcElClientSecretEl {
        IamWorkforcePoolProviderOidcElClientSecretEl {
            value: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct IamWorkforcePoolProviderOidcElClientSecretElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IamWorkforcePoolProviderOidcElClientSecretElRef {
    fn new(shared: StackShared, base: String) -> IamWorkforcePoolProviderOidcElClientSecretElRef {
        IamWorkforcePoolProviderOidcElClientSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolProviderOidcElClientSecretElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> ListRef<IamWorkforcePoolProviderOidcElClientSecretElValueElRef> {
        ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct IamWorkforcePoolProviderOidcElWebSsoConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_scopes: Option<ListField<PrimField<String>>>,
    assertion_claims_behavior: PrimField<String>,
    response_type: PrimField<String>,
}

impl IamWorkforcePoolProviderOidcElWebSsoConfigEl {
    #[doc= "Set the field `additional_scopes`.\nAdditional scopes to request for in the OIDC authentication request on top of scopes requested by default. By default, the 'openid', 'profile' and 'email' scopes that are supported by the identity provider are requested.\nEach additional scope may be at most 256 characters. A maximum of 10 additional scopes may be configured."]
    pub fn set_additional_scopes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.additional_scopes = Some(v.into());
        self
    }
}

impl ToListMappable for IamWorkforcePoolProviderOidcElWebSsoConfigEl {
    type O = BlockAssignable<IamWorkforcePoolProviderOidcElWebSsoConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolProviderOidcElWebSsoConfigEl {
    #[doc= "The behavior for how OIDC Claims are included in the 'assertion' object used for attribute mapping and attribute condition.\n* MERGE_USER_INFO_OVER_ID_TOKEN_CLAIMS: Merge the UserInfo Endpoint Claims with ID Token Claims, preferring UserInfo Claim Values for the same Claim Name. This option is available only for the Authorization Code Flow.\n* ONLY_ID_TOKEN_CLAIMS: Only include ID Token Claims. Possible values: [\"MERGE_USER_INFO_OVER_ID_TOKEN_CLAIMS\", \"ONLY_ID_TOKEN_CLAIMS\"]"]
    pub assertion_claims_behavior: PrimField<String>,
    #[doc= "The Response Type to request for in the OIDC Authorization Request for web sign-in.\n\nThe 'CODE' Response Type is recommended to avoid the Implicit Flow, for security reasons.\n* CODE: The 'response_type=code' selection uses the Authorization Code Flow for web sign-in. Requires a configured client secret.\n* ID_TOKEN: The 'response_type=id_token' selection uses the Implicit Flow for web sign-in. Possible values: [\"CODE\", \"ID_TOKEN\"]"]
    pub response_type: PrimField<String>,
}

impl BuildIamWorkforcePoolProviderOidcElWebSsoConfigEl {
    pub fn build(self) -> IamWorkforcePoolProviderOidcElWebSsoConfigEl {
        IamWorkforcePoolProviderOidcElWebSsoConfigEl {
            additional_scopes: core::default::Default::default(),
            assertion_claims_behavior: self.assertion_claims_behavior,
            response_type: self.response_type,
        }
    }
}

pub struct IamWorkforcePoolProviderOidcElWebSsoConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IamWorkforcePoolProviderOidcElWebSsoConfigElRef {
    fn new(shared: StackShared, base: String) -> IamWorkforcePoolProviderOidcElWebSsoConfigElRef {
        IamWorkforcePoolProviderOidcElWebSsoConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolProviderOidcElWebSsoConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `additional_scopes` after provisioning.\nAdditional scopes to request for in the OIDC authentication request on top of scopes requested by default. By default, the 'openid', 'profile' and 'email' scopes that are supported by the identity provider are requested.\nEach additional scope may be at most 256 characters. A maximum of 10 additional scopes may be configured."]
    pub fn additional_scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.additional_scopes", self.base))
    }

    #[doc= "Get a reference to the value of field `assertion_claims_behavior` after provisioning.\nThe behavior for how OIDC Claims are included in the 'assertion' object used for attribute mapping and attribute condition.\n* MERGE_USER_INFO_OVER_ID_TOKEN_CLAIMS: Merge the UserInfo Endpoint Claims with ID Token Claims, preferring UserInfo Claim Values for the same Claim Name. This option is available only for the Authorization Code Flow.\n* ONLY_ID_TOKEN_CLAIMS: Only include ID Token Claims. Possible values: [\"MERGE_USER_INFO_OVER_ID_TOKEN_CLAIMS\", \"ONLY_ID_TOKEN_CLAIMS\"]"]
    pub fn assertion_claims_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.assertion_claims_behavior", self.base))
    }

    #[doc= "Get a reference to the value of field `response_type` after provisioning.\nThe Response Type to request for in the OIDC Authorization Request for web sign-in.\n\nThe 'CODE' Response Type is recommended to avoid the Implicit Flow, for security reasons.\n* CODE: The 'response_type=code' selection uses the Authorization Code Flow for web sign-in. Requires a configured client secret.\n* ID_TOKEN: The 'response_type=id_token' selection uses the Implicit Flow for web sign-in. Possible values: [\"CODE\", \"ID_TOKEN\"]"]
    pub fn response_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.response_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct IamWorkforcePoolProviderOidcElDynamic {
    client_secret: Option<DynamicBlock<IamWorkforcePoolProviderOidcElClientSecretEl>>,
    web_sso_config: Option<DynamicBlock<IamWorkforcePoolProviderOidcElWebSsoConfigEl>>,
}

#[derive(Serialize)]
pub struct IamWorkforcePoolProviderOidcEl {
    client_id: PrimField<String>,
    issuer_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jwks_json: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<Vec<IamWorkforcePoolProviderOidcElClientSecretEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_sso_config: Option<Vec<IamWorkforcePoolProviderOidcElWebSsoConfigEl>>,
    dynamic: IamWorkforcePoolProviderOidcElDynamic,
}

impl IamWorkforcePoolProviderOidcEl {
    #[doc= "Set the field `jwks_json`.\nOIDC JWKs in JSON String format. For details on definition of a\nJWK, see https:tools.ietf.org/html/rfc7517. If not set, then we\nuse the 'jwks_uri' from the discovery document fetched from the\n.well-known path for the 'issuer_uri'. Currently, RSA and EC asymmetric\nkeys are supported. The JWK must use following format and include only\nthe following fields:\n'''\n{\n  \"keys\": [\n    {\n          \"kty\": \"RSA/EC\",\n          \"alg\": \"<algorithm>\",\n          \"use\": \"sig\",\n          \"kid\": \"<key-id>\",\n          \"n\": \"\",\n          \"e\": \"\",\n          \"x\": \"\",\n          \"y\": \"\",\n          \"crv\": \"\"\n    }\n  ]\n}\n'''"]
    pub fn set_jwks_json(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.jwks_json = Some(v.into());
        self
    }

    #[doc= "Set the field `client_secret`.\n"]
    pub fn set_client_secret(
        mut self,
        v: impl Into<BlockAssignable<IamWorkforcePoolProviderOidcElClientSecretEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.client_secret = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.client_secret = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `web_sso_config`.\n"]
    pub fn set_web_sso_config(
        mut self,
        v: impl Into<BlockAssignable<IamWorkforcePoolProviderOidcElWebSsoConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.web_sso_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.web_sso_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for IamWorkforcePoolProviderOidcEl {
    type O = BlockAssignable<IamWorkforcePoolProviderOidcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolProviderOidcEl {
    #[doc= "The client ID. Must match the audience claim of the JWT issued by the identity provider."]
    pub client_id: PrimField<String>,
    #[doc= "The OIDC issuer URI. Must be a valid URI using the 'https' scheme."]
    pub issuer_uri: PrimField<String>,
}

impl BuildIamWorkforcePoolProviderOidcEl {
    pub fn build(self) -> IamWorkforcePoolProviderOidcEl {
        IamWorkforcePoolProviderOidcEl {
            client_id: self.client_id,
            issuer_uri: self.issuer_uri,
            jwks_json: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            web_sso_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct IamWorkforcePoolProviderOidcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IamWorkforcePoolProviderOidcElRef {
    fn new(shared: StackShared, base: String) -> IamWorkforcePoolProviderOidcElRef {
        IamWorkforcePoolProviderOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolProviderOidcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `client_id` after provisioning.\nThe client ID. Must match the audience claim of the JWT issued by the identity provider."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc= "Get a reference to the value of field `issuer_uri` after provisioning.\nThe OIDC issuer URI. Must be a valid URI using the 'https' scheme."]
    pub fn issuer_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer_uri", self.base))
    }

    #[doc= "Get a reference to the value of field `jwks_json` after provisioning.\nOIDC JWKs in JSON String format. For details on definition of a\nJWK, see https:tools.ietf.org/html/rfc7517. If not set, then we\nuse the 'jwks_uri' from the discovery document fetched from the\n.well-known path for the 'issuer_uri'. Currently, RSA and EC asymmetric\nkeys are supported. The JWK must use following format and include only\nthe following fields:\n'''\n{\n  \"keys\": [\n    {\n          \"kty\": \"RSA/EC\",\n          \"alg\": \"<algorithm>\",\n          \"use\": \"sig\",\n          \"kid\": \"<key-id>\",\n          \"n\": \"\",\n          \"e\": \"\",\n          \"x\": \"\",\n          \"y\": \"\",\n          \"crv\": \"\"\n    }\n  ]\n}\n'''"]
    pub fn jwks_json(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jwks_json", self.base))
    }

    #[doc= "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> ListRef<IamWorkforcePoolProviderOidcElClientSecretElRef> {
        ListRef::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc= "Get a reference to the value of field `web_sso_config` after provisioning.\n"]
    pub fn web_sso_config(&self) -> ListRef<IamWorkforcePoolProviderOidcElWebSsoConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.web_sso_config", self.base))
    }
}

#[derive(Serialize)]
pub struct IamWorkforcePoolProviderSamlEl {
    idp_metadata_xml: PrimField<String>,
}

impl IamWorkforcePoolProviderSamlEl { }

impl ToListMappable for IamWorkforcePoolProviderSamlEl {
    type O = BlockAssignable<IamWorkforcePoolProviderSamlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolProviderSamlEl {
    #[doc= "SAML Identity provider configuration metadata xml doc.\nThe xml document should comply with [SAML 2.0 specification](https://docs.oasis-open.org/security/saml/v2.0/saml-metadata-2.0-os.pdf).\nThe max size of the acceptable xml document will be bounded to 128k characters.\n\nThe metadata xml document should satisfy the following constraints:\n1) Must contain an Identity Provider Entity ID.\n2) Must contain at least one non-expired signing key certificate.\n3) For each signing key:\n  a) Valid from should be no more than 7 days from now.\n  b) Valid to should be no more than 10 years in the future.\n4) Up to 3 IdP signing keys are allowed in the metadata xml.\n\nWhen updating the provider's metadata xml, at least one non-expired signing key\nmust overlap with the existing metadata. This requirement is skipped if there are\nno non-expired signing keys present in the existing metadata."]
    pub idp_metadata_xml: PrimField<String>,
}

impl BuildIamWorkforcePoolProviderSamlEl {
    pub fn build(self) -> IamWorkforcePoolProviderSamlEl {
        IamWorkforcePoolProviderSamlEl { idp_metadata_xml: self.idp_metadata_xml }
    }
}

pub struct IamWorkforcePoolProviderSamlElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IamWorkforcePoolProviderSamlElRef {
    fn new(shared: StackShared, base: String) -> IamWorkforcePoolProviderSamlElRef {
        IamWorkforcePoolProviderSamlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolProviderSamlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `idp_metadata_xml` after provisioning.\nSAML Identity provider configuration metadata xml doc.\nThe xml document should comply with [SAML 2.0 specification](https://docs.oasis-open.org/security/saml/v2.0/saml-metadata-2.0-os.pdf).\nThe max size of the acceptable xml document will be bounded to 128k characters.\n\nThe metadata xml document should satisfy the following constraints:\n1) Must contain an Identity Provider Entity ID.\n2) Must contain at least one non-expired signing key certificate.\n3) For each signing key:\n  a) Valid from should be no more than 7 days from now.\n  b) Valid to should be no more than 10 years in the future.\n4) Up to 3 IdP signing keys are allowed in the metadata xml.\n\nWhen updating the provider's metadata xml, at least one non-expired signing key\nmust overlap with the existing metadata. This requirement is skipped if there are\nno non-expired signing keys present in the existing metadata."]
    pub fn idp_metadata_xml(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.idp_metadata_xml", self.base))
    }
}

#[derive(Serialize)]
pub struct IamWorkforcePoolProviderTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl IamWorkforcePoolProviderTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for IamWorkforcePoolProviderTimeoutsEl {
    type O = BlockAssignable<IamWorkforcePoolProviderTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolProviderTimeoutsEl {}

impl BuildIamWorkforcePoolProviderTimeoutsEl {
    pub fn build(self) -> IamWorkforcePoolProviderTimeoutsEl {
        IamWorkforcePoolProviderTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct IamWorkforcePoolProviderTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IamWorkforcePoolProviderTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IamWorkforcePoolProviderTimeoutsElRef {
        IamWorkforcePoolProviderTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolProviderTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct IamWorkforcePoolProviderDynamic {
    oidc: Option<DynamicBlock<IamWorkforcePoolProviderOidcEl>>,
    saml: Option<DynamicBlock<IamWorkforcePoolProviderSamlEl>>,
}
