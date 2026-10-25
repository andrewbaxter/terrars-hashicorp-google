#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IamWorkloadIdentityPoolProviderData_ {
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
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    workload_identity_pool_id: terrars::PrimField<std::string::String>,
    workload_identity_pool_provider_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws: std::option::Option<std::vec::Vec<IamWorkloadIdentityPoolProvider_Aws>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc: std::option::Option<std::vec::Vec<IamWorkloadIdentityPoolProvider_Oidc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saml: std::option::Option<std::vec::Vec<IamWorkloadIdentityPoolProvider_Saml>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IamWorkloadIdentityPoolProvider_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    x509: std::option::Option<std::vec::Vec<IamWorkloadIdentityPoolProvider_X509>>,
    dynamic: IamWorkloadIdentityPoolProviderDynamic,
}

struct IamWorkloadIdentityPoolProvider_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IamWorkloadIdentityPoolProviderData_>,
}

#[derive(Clone)]
pub struct IamWorkloadIdentityPoolProvider(std::rc::Rc<IamWorkloadIdentityPoolProvider_>);

impl IamWorkloadIdentityPoolProvider {
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
        "Set the field `attribute_condition`.\n[A Common Expression Language](https://github.com/google/cel-spec) expression, in\nplain text, to restrict what otherwise valid authentication credentials issued by the\nprovider should not be accepted.\n\nThe expression must output a boolean representing whether to allow the federation.\n\nThe following keywords may be referenced in the expressions:\n  * 'assertion': JSON representing the authentication credential issued by the provider.\n  * 'google': The Google attributes mapped from the assertion in the 'attribute_mappings'.\n  * 'attribute': The custom attributes mapped from the assertion in the 'attribute_mappings'.\n\nThe maximum length of the attribute condition expression is 4096 characters. If\nunspecified, all valid authentication credential are accepted.\n\nThe following example shows how to only allow credentials with a mapped 'google.groups'\nvalue of 'admins':\n'''\n\"'admins' in google.groups\"\n'''"]
    pub fn set_attribute_condition(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().attribute_condition = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `attribute_mapping`.\nMaps attributes from authentication credentials issued by an external identity provider\nto Google Cloud attributes, such as 'subject' and 'segment'.\n\nEach key must be a string specifying the Google Cloud IAM attribute to map to.\n\nThe following keys are supported:\n  * 'google.subject': The principal IAM is authenticating. You can reference this value\n    in IAM bindings. This is also the subject that appears in Cloud Logging logs.\n    Cannot exceed 127 characters.\n  * 'google.groups': Groups the external identity belongs to. You can grant groups\n    access to resources using an IAM 'principalSet' binding; access applies to all\n    members of the group.\n\nYou can also provide custom attributes by specifying 'attribute.{custom_attribute}',\nwhere '{custom_attribute}' is the name of the custom attribute to be mapped. You can\ndefine a maximum of 50 custom attributes. The maximum length of a mapped attribute key\nis 100 characters, and the key may only contain the characters [a-z0-9_].\n\nYou can reference these attributes in IAM policies to define fine-grained access for a\nworkload to Google Cloud resources. For example:\n  * 'google.subject':\n    'principal://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/subject/{value}'\n  * 'google.groups':\n    'principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/group/{value}'\n  * 'attribute.{custom_attribute}':\n    'principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/attribute.{custom_attribute}/{value}'\n\nEach value must be a [Common Expression Language](https://github.com/google/cel-spec)\nfunction that maps an identity provider credential to the normalized attribute specified\nby the corresponding map key.\n\nYou can use the 'assertion' keyword in the expression to access a JSON representation of\nthe authentication credential issued by the provider.\n\nThe maximum length of an attribute mapping expression is 2048 characters. When evaluated,\nthe total size of all mapped attributes must not exceed 8KB.\n\nFor AWS providers, the following rules apply:\n  - If no attribute mapping is defined, the following default mapping applies:\n    '''\n    {\n      \"google.subject\":\"assertion.arn\",\n      \"attribute.aws_role\":\n        \"assertion.arn.contains('assumed-role')\"\n        \" ? assertion.arn.extract('{account_arn}assumed-role/')\"\n        \"   + 'assumed-role/'\"\n        \"   + assertion.arn.extract('assumed-role/{role_name}/')\"\n        \" : assertion.arn\",\n    }\n    '''\n  - If any custom attribute mappings are defined, they must include a mapping to the\n    'google.subject' attribute.\n\nFor OIDC providers, the following rules apply:\n  - Custom attribute mappings must be defined, and must include a mapping to the\n    'google.subject' attribute. For example, the following maps the 'sub' claim of the\n    incoming credential to the 'subject' attribute on a Google token.\n    '''\n    {\"google.subject\": \"assertion.sub\"}\n    '''"]
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

    #[doc = "Set the field `description`.\nA description for the provider. Cannot exceed 256 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nWhether the provider is disabled. You cannot use a disabled provider to exchange tokens.\nHowever, existing tokens still grant access."]
    pub fn set_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nA display name for the provider. Cannot exceed 32 characters."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws`.\n"]
    pub fn set_aws(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamWorkloadIdentityPoolProvider_Aws>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().aws = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.aws = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oidc`.\n"]
    pub fn set_oidc(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamWorkloadIdentityPoolProvider_Oidc>>,
    ) -> Self {
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
    pub fn set_saml(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamWorkloadIdentityPoolProvider_Saml>>,
    ) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<IamWorkloadIdentityPoolProvider_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `x509`.\n"]
    pub fn set_x509(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamWorkloadIdentityPoolProvider_X509>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().x509 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.x509 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `attribute_condition` after provisioning.\n[A Common Expression Language](https://github.com/google/cel-spec) expression, in\nplain text, to restrict what otherwise valid authentication credentials issued by the\nprovider should not be accepted.\n\nThe expression must output a boolean representing whether to allow the federation.\n\nThe following keywords may be referenced in the expressions:\n  * 'assertion': JSON representing the authentication credential issued by the provider.\n  * 'google': The Google attributes mapped from the assertion in the 'attribute_mappings'.\n  * 'attribute': The custom attributes mapped from the assertion in the 'attribute_mappings'.\n\nThe maximum length of the attribute condition expression is 4096 characters. If\nunspecified, all valid authentication credential are accepted.\n\nThe following example shows how to only allow credentials with a mapped 'google.groups'\nvalue of 'admins':\n'''\n\"'admins' in google.groups\"\n'''"]
    pub fn attribute_condition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_condition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `attribute_mapping` after provisioning.\nMaps attributes from authentication credentials issued by an external identity provider\nto Google Cloud attributes, such as 'subject' and 'segment'.\n\nEach key must be a string specifying the Google Cloud IAM attribute to map to.\n\nThe following keys are supported:\n  * 'google.subject': The principal IAM is authenticating. You can reference this value\n    in IAM bindings. This is also the subject that appears in Cloud Logging logs.\n    Cannot exceed 127 characters.\n  * 'google.groups': Groups the external identity belongs to. You can grant groups\n    access to resources using an IAM 'principalSet' binding; access applies to all\n    members of the group.\n\nYou can also provide custom attributes by specifying 'attribute.{custom_attribute}',\nwhere '{custom_attribute}' is the name of the custom attribute to be mapped. You can\ndefine a maximum of 50 custom attributes. The maximum length of a mapped attribute key\nis 100 characters, and the key may only contain the characters [a-z0-9_].\n\nYou can reference these attributes in IAM policies to define fine-grained access for a\nworkload to Google Cloud resources. For example:\n  * 'google.subject':\n    'principal://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/subject/{value}'\n  * 'google.groups':\n    'principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/group/{value}'\n  * 'attribute.{custom_attribute}':\n    'principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/attribute.{custom_attribute}/{value}'\n\nEach value must be a [Common Expression Language](https://github.com/google/cel-spec)\nfunction that maps an identity provider credential to the normalized attribute specified\nby the corresponding map key.\n\nYou can use the 'assertion' keyword in the expression to access a JSON representation of\nthe authentication credential issued by the provider.\n\nThe maximum length of an attribute mapping expression is 2048 characters. When evaluated,\nthe total size of all mapped attributes must not exceed 8KB.\n\nFor AWS providers, the following rules apply:\n  - If no attribute mapping is defined, the following default mapping applies:\n    '''\n    {\n      \"google.subject\":\"assertion.arn\",\n      \"attribute.aws_role\":\n        \"assertion.arn.contains('assumed-role')\"\n        \" ? assertion.arn.extract('{account_arn}assumed-role/')\"\n        \"   + 'assumed-role/'\"\n        \"   + assertion.arn.extract('assumed-role/{role_name}/')\"\n        \" : assertion.arn\",\n    }\n    '''\n  - If any custom attribute mappings are defined, they must include a mapping to the\n    'google.subject' attribute.\n\nFor OIDC providers, the following rules apply:\n  - Custom attribute mappings must be defined, and must include a mapping to the\n    'google.subject' attribute. For example, the following maps the 'sub' claim of the\n    incoming credential to the 'subject' attribute on a Google token.\n    '''\n    {\"google.subject\": \"assertion.sub\"}\n    '''"]
    pub fn attribute_mapping(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.attribute_mapping", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description for the provider. Cannot exceed 256 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the provider is disabled. You cannot use a disabled provider to exchange tokens.\nHowever, existing tokens still grant access."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA display name for the provider. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the provider as\n'projects/{project_number}/locations/global/workloadIdentityPools/{workload_identity_pool_id}/providers/{workload_identity_pool_provider_id}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the provider.\n* STATE_UNSPECIFIED: State unspecified.\n* ACTIVE: The provider is active, and may be used to validate authentication credentials.\n* DELETED: The provider is soft-deleted. Soft-deleted providers are permanently deleted\n  after approximately 30 days. You can restore a soft-deleted provider using\n  UndeleteWorkloadIdentityPoolProvider. You cannot reuse the ID of a soft-deleted provider\n  until it is permanently deleted."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_identity_pool_id` after provisioning.\nThe ID used for the pool, which is the final component of the pool resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workload_identity_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workload_identity_pool_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_identity_pool_provider_id` after provisioning.\nThe ID for the provider, which becomes the final component of the resource name. This\nvalue must be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workload_identity_pool_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_identity_pool_provider_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `aws` after provisioning.\n"]
    pub fn aws(&self) -> terrars::ListRef<IamWorkloadIdentityPoolProvider_AwsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> terrars::ListRef<IamWorkloadIdentityPoolProvider_OidcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> terrars::ListRef<IamWorkloadIdentityPoolProvider_SamlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkloadIdentityPoolProvider_TimeoutsRef {
        IamWorkloadIdentityPoolProvider_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `x509` after provisioning.\n"]
    pub fn x509(&self) -> terrars::ListRef<IamWorkloadIdentityPoolProvider_X509Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.x509", self.extract_ref()))
    }
}

impl terrars::Referable for IamWorkloadIdentityPoolProvider {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IamWorkloadIdentityPoolProvider { }

impl terrars::ToListMappable for IamWorkloadIdentityPoolProvider {
    type O = terrars::ListRef<IamWorkloadIdentityPoolProviderRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IamWorkloadIdentityPoolProvider_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_iam_workload_identity_pool_provider".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIamWorkloadIdentityPoolProvider {
    pub tf_id: std::string::String,
    #[doc =
        "The ID used for the pool, which is the final component of the pool resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub workload_identity_pool_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID for the provider, which becomes the final component of the resource name. This\nvalue must be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub workload_identity_pool_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildIamWorkloadIdentityPoolProvider {
    pub fn build(self, stack: &mut terrars::Stack) -> IamWorkloadIdentityPoolProvider {
        let out = IamWorkloadIdentityPoolProvider(std::rc::Rc::new(IamWorkloadIdentityPoolProvider_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IamWorkloadIdentityPoolProviderData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                attribute_condition: core::default::Default::default(),
                attribute_mapping: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disabled: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                workload_identity_pool_id: self.workload_identity_pool_id,
                workload_identity_pool_provider_id: self.workload_identity_pool_provider_id,
                aws: core::default::Default::default(),
                oidc: core::default::Default::default(),
                saml: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                x509: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IamWorkloadIdentityPoolProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPoolProviderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IamWorkloadIdentityPoolProviderRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attribute_condition` after provisioning.\n[A Common Expression Language](https://github.com/google/cel-spec) expression, in\nplain text, to restrict what otherwise valid authentication credentials issued by the\nprovider should not be accepted.\n\nThe expression must output a boolean representing whether to allow the federation.\n\nThe following keywords may be referenced in the expressions:\n  * 'assertion': JSON representing the authentication credential issued by the provider.\n  * 'google': The Google attributes mapped from the assertion in the 'attribute_mappings'.\n  * 'attribute': The custom attributes mapped from the assertion in the 'attribute_mappings'.\n\nThe maximum length of the attribute condition expression is 4096 characters. If\nunspecified, all valid authentication credential are accepted.\n\nThe following example shows how to only allow credentials with a mapped 'google.groups'\nvalue of 'admins':\n'''\n\"'admins' in google.groups\"\n'''"]
    pub fn attribute_condition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_condition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `attribute_mapping` after provisioning.\nMaps attributes from authentication credentials issued by an external identity provider\nto Google Cloud attributes, such as 'subject' and 'segment'.\n\nEach key must be a string specifying the Google Cloud IAM attribute to map to.\n\nThe following keys are supported:\n  * 'google.subject': The principal IAM is authenticating. You can reference this value\n    in IAM bindings. This is also the subject that appears in Cloud Logging logs.\n    Cannot exceed 127 characters.\n  * 'google.groups': Groups the external identity belongs to. You can grant groups\n    access to resources using an IAM 'principalSet' binding; access applies to all\n    members of the group.\n\nYou can also provide custom attributes by specifying 'attribute.{custom_attribute}',\nwhere '{custom_attribute}' is the name of the custom attribute to be mapped. You can\ndefine a maximum of 50 custom attributes. The maximum length of a mapped attribute key\nis 100 characters, and the key may only contain the characters [a-z0-9_].\n\nYou can reference these attributes in IAM policies to define fine-grained access for a\nworkload to Google Cloud resources. For example:\n  * 'google.subject':\n    'principal://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/subject/{value}'\n  * 'google.groups':\n    'principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/group/{value}'\n  * 'attribute.{custom_attribute}':\n    'principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/attribute.{custom_attribute}/{value}'\n\nEach value must be a [Common Expression Language](https://github.com/google/cel-spec)\nfunction that maps an identity provider credential to the normalized attribute specified\nby the corresponding map key.\n\nYou can use the 'assertion' keyword in the expression to access a JSON representation of\nthe authentication credential issued by the provider.\n\nThe maximum length of an attribute mapping expression is 2048 characters. When evaluated,\nthe total size of all mapped attributes must not exceed 8KB.\n\nFor AWS providers, the following rules apply:\n  - If no attribute mapping is defined, the following default mapping applies:\n    '''\n    {\n      \"google.subject\":\"assertion.arn\",\n      \"attribute.aws_role\":\n        \"assertion.arn.contains('assumed-role')\"\n        \" ? assertion.arn.extract('{account_arn}assumed-role/')\"\n        \"   + 'assumed-role/'\"\n        \"   + assertion.arn.extract('assumed-role/{role_name}/')\"\n        \" : assertion.arn\",\n    }\n    '''\n  - If any custom attribute mappings are defined, they must include a mapping to the\n    'google.subject' attribute.\n\nFor OIDC providers, the following rules apply:\n  - Custom attribute mappings must be defined, and must include a mapping to the\n    'google.subject' attribute. For example, the following maps the 'sub' claim of the\n    incoming credential to the 'subject' attribute on a Google token.\n    '''\n    {\"google.subject\": \"assertion.sub\"}\n    '''"]
    pub fn attribute_mapping(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.attribute_mapping", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description for the provider. Cannot exceed 256 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the provider is disabled. You cannot use a disabled provider to exchange tokens.\nHowever, existing tokens still grant access."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA display name for the provider. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the provider as\n'projects/{project_number}/locations/global/workloadIdentityPools/{workload_identity_pool_id}/providers/{workload_identity_pool_provider_id}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the provider.\n* STATE_UNSPECIFIED: State unspecified.\n* ACTIVE: The provider is active, and may be used to validate authentication credentials.\n* DELETED: The provider is soft-deleted. Soft-deleted providers are permanently deleted\n  after approximately 30 days. You can restore a soft-deleted provider using\n  UndeleteWorkloadIdentityPoolProvider. You cannot reuse the ID of a soft-deleted provider\n  until it is permanently deleted."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_identity_pool_id` after provisioning.\nThe ID used for the pool, which is the final component of the pool resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workload_identity_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workload_identity_pool_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_identity_pool_provider_id` after provisioning.\nThe ID for the provider, which becomes the final component of the resource name. This\nvalue must be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workload_identity_pool_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_identity_pool_provider_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `aws` after provisioning.\n"]
    pub fn aws(&self) -> terrars::ListRef<IamWorkloadIdentityPoolProvider_AwsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> terrars::ListRef<IamWorkloadIdentityPoolProvider_OidcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> terrars::ListRef<IamWorkloadIdentityPoolProvider_SamlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkloadIdentityPoolProvider_TimeoutsRef {
        IamWorkloadIdentityPoolProvider_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `x509` after provisioning.\n"]
    pub fn x509(&self) -> terrars::ListRef<IamWorkloadIdentityPoolProvider_X509Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.x509", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct IamWorkloadIdentityPoolProviderDynamic {
    aws: std::option::Option<terrars::DynamicBlock<IamWorkloadIdentityPoolProvider_Aws>>,
    oidc: std::option::Option<terrars::DynamicBlock<IamWorkloadIdentityPoolProvider_Oidc>>,
    saml: std::option::Option<terrars::DynamicBlock<IamWorkloadIdentityPoolProvider_Saml>>,
    x509: std::option::Option<terrars::DynamicBlock<IamWorkloadIdentityPoolProvider_X509>>,
}

#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolProvider_Aws {
    account_id: terrars::PrimField<std::string::String>,
}

impl IamWorkloadIdentityPoolProvider_Aws { }

impl terrars::ToListMappable for IamWorkloadIdentityPoolProvider_Aws {
    type O = terrars::BlockAssignable<IamWorkloadIdentityPoolProvider_Aws>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkloadIdentityPoolProvider_Aws {
    #[doc = "The AWS account ID."]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildIamWorkloadIdentityPoolProvider_Aws {
    pub fn build(self) -> IamWorkloadIdentityPoolProvider_Aws {
        IamWorkloadIdentityPoolProvider_Aws { account_id: self.account_id }
    }
}

#[allow(dead_code)]
pub struct IamWorkloadIdentityPoolProvider_AwsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPoolProvider_AwsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamWorkloadIdentityPoolProvider_AwsRef {
        IamWorkloadIdentityPoolProvider_AwsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkloadIdentityPoolProvider_AwsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe AWS account ID."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }
}

#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolProvider_Oidc {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_audiences: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    issuer_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwks_json: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IamWorkloadIdentityPoolProvider_Oidc {
    #[doc =
        "Set the field `allowed_audiences`.\nAcceptable values for the 'aud' field (audience) in the OIDC token. Token exchange\nrequests are rejected if the token audience does not match one of the configured\nvalues. Each audience may be at most 256 characters. A maximum of 10 audiences may\nbe configured.\n\nIf this list is empty, the OIDC token audience must be equal to the full canonical\nresource name of the WorkloadIdentityPoolProvider, with or without the HTTPS prefix.\nFor example:\n'''\n//iam.googleapis.com/projects/<project-number>/locations/<location>/workloadIdentityPools/<pool-id>/providers/<provider-id>\nhttps://iam.googleapis.com/projects/<project-number>/locations/<location>/workloadIdentityPools/<pool-id>/providers/<provider-id>\n'''"]
    pub fn set_allowed_audiences(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_audiences = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jwks_json`.\nOIDC JWKs in JSON String format. For details on definition of a\nJWK, see https:tools.ietf.org/html/rfc7517. If not set, then we\nuse the 'jwks_uri' from the discovery document fetched from the\n.well-known path for the 'issuer_uri'. Currently, RSA and EC asymmetric\nkeys are supported. The JWK must use following format and include only\nthe following fields:\n'''\n{\n  \"keys\": [\n    {\n          \"kty\": \"RSA/EC\",\n          \"alg\": \"<algorithm>\",\n          \"use\": \"sig\",\n          \"kid\": \"<key-id>\",\n          \"n\": \"\",\n          \"e\": \"\",\n          \"x\": \"\",\n          \"y\": \"\",\n          \"crv\": \"\"\n    }\n  ]\n}\n'''"]
    pub fn set_jwks_json(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.jwks_json = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IamWorkloadIdentityPoolProvider_Oidc {
    type O = terrars::BlockAssignable<IamWorkloadIdentityPoolProvider_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkloadIdentityPoolProvider_Oidc {
    #[doc = "The OIDC issuer URL."]
    pub issuer_uri: terrars::PrimField<std::string::String>,
}

impl BuildIamWorkloadIdentityPoolProvider_Oidc {
    pub fn build(self) -> IamWorkloadIdentityPoolProvider_Oidc {
        IamWorkloadIdentityPoolProvider_Oidc {
            allowed_audiences: core::default::Default::default(),
            issuer_uri: self.issuer_uri,
            jwks_json: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkloadIdentityPoolProvider_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPoolProvider_OidcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamWorkloadIdentityPoolProvider_OidcRef {
        IamWorkloadIdentityPoolProvider_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkloadIdentityPoolProvider_OidcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_audiences` after provisioning.\nAcceptable values for the 'aud' field (audience) in the OIDC token. Token exchange\nrequests are rejected if the token audience does not match one of the configured\nvalues. Each audience may be at most 256 characters. A maximum of 10 audiences may\nbe configured.\n\nIf this list is empty, the OIDC token audience must be equal to the full canonical\nresource name of the WorkloadIdentityPoolProvider, with or without the HTTPS prefix.\nFor example:\n'''\n//iam.googleapis.com/projects/<project-number>/locations/<location>/workloadIdentityPools/<pool-id>/providers/<provider-id>\nhttps://iam.googleapis.com/projects/<project-number>/locations/<location>/workloadIdentityPools/<pool-id>/providers/<provider-id>\n'''"]
    pub fn allowed_audiences(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_audiences", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer_uri` after provisioning.\nThe OIDC issuer URL."]
    pub fn issuer_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jwks_json` after provisioning.\nOIDC JWKs in JSON String format. For details on definition of a\nJWK, see https:tools.ietf.org/html/rfc7517. If not set, then we\nuse the 'jwks_uri' from the discovery document fetched from the\n.well-known path for the 'issuer_uri'. Currently, RSA and EC asymmetric\nkeys are supported. The JWK must use following format and include only\nthe following fields:\n'''\n{\n  \"keys\": [\n    {\n          \"kty\": \"RSA/EC\",\n          \"alg\": \"<algorithm>\",\n          \"use\": \"sig\",\n          \"kid\": \"<key-id>\",\n          \"n\": \"\",\n          \"e\": \"\",\n          \"x\": \"\",\n          \"y\": \"\",\n          \"crv\": \"\"\n    }\n  ]\n}\n'''"]
    pub fn jwks_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwks_json", self.base))
    }
}

#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolProvider_Saml {
    idp_metadata_xml: terrars::PrimField<std::string::String>,
}

impl IamWorkloadIdentityPoolProvider_Saml { }

impl terrars::ToListMappable for IamWorkloadIdentityPoolProvider_Saml {
    type O = terrars::BlockAssignable<IamWorkloadIdentityPoolProvider_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkloadIdentityPoolProvider_Saml {
    #[doc = "SAML Identity provider configuration metadata xml doc."]
    pub idp_metadata_xml: terrars::PrimField<std::string::String>,
}

impl BuildIamWorkloadIdentityPoolProvider_Saml {
    pub fn build(self) -> IamWorkloadIdentityPoolProvider_Saml {
        IamWorkloadIdentityPoolProvider_Saml { idp_metadata_xml: self.idp_metadata_xml }
    }
}

#[allow(dead_code)]
pub struct IamWorkloadIdentityPoolProvider_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPoolProvider_SamlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamWorkloadIdentityPoolProvider_SamlRef {
        IamWorkloadIdentityPoolProvider_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkloadIdentityPoolProvider_SamlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `idp_metadata_xml` after provisioning.\nSAML Identity provider configuration metadata xml doc."]
    pub fn idp_metadata_xml(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idp_metadata_xml", self.base))
    }
}

#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolProvider_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IamWorkloadIdentityPoolProvider_Timeouts {
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

impl terrars::ToListMappable for IamWorkloadIdentityPoolProvider_Timeouts {
    type O = terrars::BlockAssignable<IamWorkloadIdentityPoolProvider_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkloadIdentityPoolProvider_Timeouts {}

impl BuildIamWorkloadIdentityPoolProvider_Timeouts {
    pub fn build(self) -> IamWorkloadIdentityPoolProvider_Timeouts {
        IamWorkloadIdentityPoolProvider_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkloadIdentityPoolProvider_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPoolProvider_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamWorkloadIdentityPoolProvider_TimeoutsRef {
        IamWorkloadIdentityPoolProvider_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkloadIdentityPoolProvider_TimeoutsRef {
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
pub struct IamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pem_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas {
    #[doc =
        "Set the field `pem_certificate`.\nPEM certificate of the PKI used for validation. Must only contain one\nca certificate(either root or intermediate cert)."]
    pub fn set_pem_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pem_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas {
    type O = terrars::BlockAssignable<IamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas {}

impl BuildIamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas {
    pub fn build(self) -> IamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas {
        IamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas {
            pem_certificate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCasRef {
        IamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pem_certificate` after provisioning.\nPEM certificate of the PKI used for validation. Must only contain one\nca certificate(either root or intermediate cert)."]
    pub fn pem_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pem_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors {
    #[doc =
        "Set the field `pem_certificate`.\nPEM certificate of the PKI used for validation. Must only contain one\nca certificate(either root or intermediate cert)."]
    pub fn set_pem_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pem_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors {
    type O = terrars::BlockAssignable<IamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors {}

impl BuildIamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors {
    pub fn build(self) -> IamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors {
        IamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors {
            pem_certificate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchorsRef {
        IamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pem_certificate` after provisioning.\nPEM certificate of the PKI used for validation. Must only contain one\nca certificate(either root or intermediate cert)."]
    pub fn pem_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_certificate", self.base))
    }
}

#[derive(Serialize, Default)]
struct IamWorkloadIdentityPoolProvider_X509_TrustStoreDynamic {
    intermediate_cas: std
    ::option
    ::Option<terrars::DynamicBlock<IamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas>>,
    trust_anchors: std
    ::option
    ::Option<terrars::DynamicBlock<IamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors>>,
}

#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolProvider_X509_TrustStore {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intermediate_cas: std::option::Option<std::vec::Vec<IamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_anchors: std::option::Option<std::vec::Vec<IamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors>>,
    dynamic: IamWorkloadIdentityPoolProvider_X509_TrustStoreDynamic,
}

impl IamWorkloadIdentityPoolProvider_X509_TrustStore {
    #[doc = "Set the field `intermediate_cas`.\n"]
    pub fn set_intermediate_cas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.intermediate_cas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.intermediate_cas = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trust_anchors`.\n"]
    pub fn set_trust_anchors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trust_anchors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trust_anchors = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IamWorkloadIdentityPoolProvider_X509_TrustStore {
    type O = terrars::BlockAssignable<IamWorkloadIdentityPoolProvider_X509_TrustStore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkloadIdentityPoolProvider_X509_TrustStore {}

impl BuildIamWorkloadIdentityPoolProvider_X509_TrustStore {
    pub fn build(self) -> IamWorkloadIdentityPoolProvider_X509_TrustStore {
        IamWorkloadIdentityPoolProvider_X509_TrustStore {
            intermediate_cas: core::default::Default::default(),
            trust_anchors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkloadIdentityPoolProvider_X509_TrustStoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPoolProvider_X509_TrustStoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamWorkloadIdentityPoolProvider_X509_TrustStoreRef {
        IamWorkloadIdentityPoolProvider_X509_TrustStoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkloadIdentityPoolProvider_X509_TrustStoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `intermediate_cas` after provisioning.\n"]
    pub fn intermediate_cas(
        &self,
    ) -> terrars::ListRef<IamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.intermediate_cas", self.base))
    }

    #[doc = "Get a reference to the value of field `trust_anchors` after provisioning.\n"]
    pub fn trust_anchors(&self) -> terrars::ListRef<IamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust_anchors", self.base))
    }
}

#[derive(Serialize, Default)]
struct IamWorkloadIdentityPoolProvider_X509Dynamic {
    trust_store: std::option::Option<terrars::DynamicBlock<IamWorkloadIdentityPoolProvider_X509_TrustStore>>,
}

#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolProvider_X509 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store: std::option::Option<std::vec::Vec<IamWorkloadIdentityPoolProvider_X509_TrustStore>>,
    dynamic: IamWorkloadIdentityPoolProvider_X509Dynamic,
}

impl IamWorkloadIdentityPoolProvider_X509 {
    #[doc = "Set the field `trust_store`.\n"]
    pub fn set_trust_store(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamWorkloadIdentityPoolProvider_X509_TrustStore>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trust_store = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trust_store = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IamWorkloadIdentityPoolProvider_X509 {
    type O = terrars::BlockAssignable<IamWorkloadIdentityPoolProvider_X509>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkloadIdentityPoolProvider_X509 {}

impl BuildIamWorkloadIdentityPoolProvider_X509 {
    pub fn build(self) -> IamWorkloadIdentityPoolProvider_X509 {
        IamWorkloadIdentityPoolProvider_X509 {
            trust_store: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkloadIdentityPoolProvider_X509Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPoolProvider_X509Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamWorkloadIdentityPoolProvider_X509Ref {
        IamWorkloadIdentityPoolProvider_X509Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkloadIdentityPoolProvider_X509Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `trust_store` after provisioning.\n"]
    pub fn trust_store(&self) -> terrars::ListRef<IamWorkloadIdentityPoolProvider_X509_TrustStoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust_store", self.base))
    }
}
