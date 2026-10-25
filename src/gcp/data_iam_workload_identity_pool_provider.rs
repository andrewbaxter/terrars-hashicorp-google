#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataIamWorkloadIdentityPoolProviderData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    workload_identity_pool_id: terrars::PrimField<std::string::String>,
    workload_identity_pool_provider_id: terrars::PrimField<std::string::String>,
}

struct DataIamWorkloadIdentityPoolProvider_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataIamWorkloadIdentityPoolProviderData_>,
}

#[derive(Clone)]
pub struct DataIamWorkloadIdentityPoolProvider(std::rc::Rc<DataIamWorkloadIdentityPoolProvider_>);

impl DataIamWorkloadIdentityPoolProvider {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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
        "Get a reference to the value of field `aws` after provisioning.\nAn Amazon Web Services identity provider. Not compatible with the property oidc or saml."]
    pub fn aws(&self) -> terrars::ListRef<DataIamWorkloadIdentityPoolProvider_AwsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `oidc` after provisioning.\nAn OpenId Connect 1.0 identity provider. Not compatible with the property aws or saml."]
    pub fn oidc(&self) -> terrars::ListRef<DataIamWorkloadIdentityPoolProvider_OidcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `saml` after provisioning.\nAn SAML 2.0 identity provider. Not compatible with the property oidc or aws."]
    pub fn saml(&self) -> terrars::ListRef<DataIamWorkloadIdentityPoolProvider_SamlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `x509` after provisioning.\nAn X.509-type identity provider represents a CA. It is trusted to assert a\nclient identity if the client has a certificate that chains up to this CA."]
    pub fn x509(&self) -> terrars::ListRef<DataIamWorkloadIdentityPoolProvider_X509Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.x509", self.extract_ref()))
    }
}

impl terrars::Referable for DataIamWorkloadIdentityPoolProvider {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataIamWorkloadIdentityPoolProvider { }

impl terrars::ToListMappable for DataIamWorkloadIdentityPoolProvider {
    type O = terrars::ListRef<DataIamWorkloadIdentityPoolProviderRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataIamWorkloadIdentityPoolProvider_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_iam_workload_identity_pool_provider".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIamWorkloadIdentityPoolProvider {
    pub tf_id: std::string::String,
    #[doc =
        "The ID used for the pool, which is the final component of the pool resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub workload_identity_pool_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID for the provider, which becomes the final component of the resource name. This\nvalue must be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub workload_identity_pool_provider_id: terrars::PrimField<std::string::String>,
}

impl BuildDataIamWorkloadIdentityPoolProvider {
    pub fn build(self, stack: &mut terrars::Stack) -> DataIamWorkloadIdentityPoolProvider {
        let out = DataIamWorkloadIdentityPoolProvider(std::rc::Rc::new(DataIamWorkloadIdentityPoolProvider_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataIamWorkloadIdentityPoolProviderData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                workload_identity_pool_id: self.workload_identity_pool_id,
                workload_identity_pool_provider_id: self.workload_identity_pool_provider_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIamWorkloadIdentityPoolProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamWorkloadIdentityPoolProviderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIamWorkloadIdentityPoolProviderRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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
        "Get a reference to the value of field `aws` after provisioning.\nAn Amazon Web Services identity provider. Not compatible with the property oidc or saml."]
    pub fn aws(&self) -> terrars::ListRef<DataIamWorkloadIdentityPoolProvider_AwsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `oidc` after provisioning.\nAn OpenId Connect 1.0 identity provider. Not compatible with the property aws or saml."]
    pub fn oidc(&self) -> terrars::ListRef<DataIamWorkloadIdentityPoolProvider_OidcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `saml` after provisioning.\nAn SAML 2.0 identity provider. Not compatible with the property oidc or aws."]
    pub fn saml(&self) -> terrars::ListRef<DataIamWorkloadIdentityPoolProvider_SamlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `x509` after provisioning.\nAn X.509-type identity provider represents a CA. It is trusted to assert a\nclient identity if the client has a certificate that chains up to this CA."]
    pub fn x509(&self) -> terrars::ListRef<DataIamWorkloadIdentityPoolProvider_X509Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.x509", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolProvider_Aws {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIamWorkloadIdentityPoolProvider_Aws {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamWorkloadIdentityPoolProvider_Aws {
    type O = terrars::BlockAssignable<DataIamWorkloadIdentityPoolProvider_Aws>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamWorkloadIdentityPoolProvider_Aws {}

impl BuildDataIamWorkloadIdentityPoolProvider_Aws {
    pub fn build(self) -> DataIamWorkloadIdentityPoolProvider_Aws {
        DataIamWorkloadIdentityPoolProvider_Aws { account_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataIamWorkloadIdentityPoolProvider_AwsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamWorkloadIdentityPoolProvider_AwsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIamWorkloadIdentityPoolProvider_AwsRef {
        DataIamWorkloadIdentityPoolProvider_AwsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamWorkloadIdentityPoolProvider_AwsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolProvider_Oidc {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_audiences: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwks_json: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIamWorkloadIdentityPoolProvider_Oidc {
    #[doc = "Set the field `allowed_audiences`.\n"]
    pub fn set_allowed_audiences(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_audiences = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer_uri`.\n"]
    pub fn set_issuer_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `jwks_json`.\n"]
    pub fn set_jwks_json(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.jwks_json = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamWorkloadIdentityPoolProvider_Oidc {
    type O = terrars::BlockAssignable<DataIamWorkloadIdentityPoolProvider_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamWorkloadIdentityPoolProvider_Oidc {}

impl BuildDataIamWorkloadIdentityPoolProvider_Oidc {
    pub fn build(self) -> DataIamWorkloadIdentityPoolProvider_Oidc {
        DataIamWorkloadIdentityPoolProvider_Oidc {
            allowed_audiences: core::default::Default::default(),
            issuer_uri: core::default::Default::default(),
            jwks_json: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIamWorkloadIdentityPoolProvider_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamWorkloadIdentityPoolProvider_OidcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIamWorkloadIdentityPoolProvider_OidcRef {
        DataIamWorkloadIdentityPoolProvider_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamWorkloadIdentityPoolProvider_OidcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_audiences` after provisioning.\n"]
    pub fn allowed_audiences(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_audiences", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer_uri` after provisioning.\n"]
    pub fn issuer_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `jwks_json` after provisioning.\n"]
    pub fn jwks_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwks_json", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolProvider_Saml {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idp_metadata_xml: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIamWorkloadIdentityPoolProvider_Saml {
    #[doc = "Set the field `idp_metadata_xml`.\n"]
    pub fn set_idp_metadata_xml(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.idp_metadata_xml = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamWorkloadIdentityPoolProvider_Saml {
    type O = terrars::BlockAssignable<DataIamWorkloadIdentityPoolProvider_Saml>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamWorkloadIdentityPoolProvider_Saml {}

impl BuildDataIamWorkloadIdentityPoolProvider_Saml {
    pub fn build(self) -> DataIamWorkloadIdentityPoolProvider_Saml {
        DataIamWorkloadIdentityPoolProvider_Saml { idp_metadata_xml: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataIamWorkloadIdentityPoolProvider_SamlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamWorkloadIdentityPoolProvider_SamlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIamWorkloadIdentityPoolProvider_SamlRef {
        DataIamWorkloadIdentityPoolProvider_SamlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamWorkloadIdentityPoolProvider_SamlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idp_metadata_xml` after provisioning.\n"]
    pub fn idp_metadata_xml(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idp_metadata_xml", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pem_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas {
    #[doc = "Set the field `pem_certificate`.\n"]
    pub fn set_pem_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pem_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas {
    type O = terrars::BlockAssignable<DataIamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas {}

impl BuildDataIamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas {
    pub fn build(self) -> DataIamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas {
        DataIamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas {
            pem_certificate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCasRef {
        DataIamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pem_certificate` after provisioning.\n"]
    pub fn pem_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pem_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors {
    #[doc = "Set the field `pem_certificate`.\n"]
    pub fn set_pem_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pem_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors {
    type O = terrars::BlockAssignable<DataIamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors {}

impl BuildDataIamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors {
    pub fn build(self) -> DataIamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors {
        DataIamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors {
            pem_certificate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchorsRef {
        DataIamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pem_certificate` after provisioning.\n"]
    pub fn pem_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolProvider_X509_TrustStore {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intermediate_cas: std
    ::option
    ::Option<terrars::ListField<DataIamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_anchors: std
    ::option
    ::Option<terrars::ListField<DataIamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors>>,
}

impl DataIamWorkloadIdentityPoolProvider_X509_TrustStore {
    #[doc = "Set the field `intermediate_cas`.\n"]
    pub fn set_intermediate_cas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataIamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCas>>,
    ) -> Self {
        self.intermediate_cas = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_anchors`.\n"]
    pub fn set_trust_anchors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataIamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchors>>,
    ) -> Self {
        self.trust_anchors = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamWorkloadIdentityPoolProvider_X509_TrustStore {
    type O = terrars::BlockAssignable<DataIamWorkloadIdentityPoolProvider_X509_TrustStore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamWorkloadIdentityPoolProvider_X509_TrustStore {}

impl BuildDataIamWorkloadIdentityPoolProvider_X509_TrustStore {
    pub fn build(self) -> DataIamWorkloadIdentityPoolProvider_X509_TrustStore {
        DataIamWorkloadIdentityPoolProvider_X509_TrustStore {
            intermediate_cas: core::default::Default::default(),
            trust_anchors: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIamWorkloadIdentityPoolProvider_X509_TrustStoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamWorkloadIdentityPoolProvider_X509_TrustStoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIamWorkloadIdentityPoolProvider_X509_TrustStoreRef {
        DataIamWorkloadIdentityPoolProvider_X509_TrustStoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamWorkloadIdentityPoolProvider_X509_TrustStoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `intermediate_cas` after provisioning.\n"]
    pub fn intermediate_cas(
        &self,
    ) -> terrars::ListRef<DataIamWorkloadIdentityPoolProvider_X509_TrustStore_IntermediateCasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.intermediate_cas", self.base))
    }

    #[doc = "Get a reference to the value of field `trust_anchors` after provisioning.\n"]
    pub fn trust_anchors(&self) -> terrars::ListRef<DataIamWorkloadIdentityPoolProvider_X509_TrustStore_TrustAnchorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust_anchors", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolProvider_X509 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store: std::option::Option<terrars::ListField<DataIamWorkloadIdentityPoolProvider_X509_TrustStore>>,
}

impl DataIamWorkloadIdentityPoolProvider_X509 {
    #[doc = "Set the field `trust_store`.\n"]
    pub fn set_trust_store(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataIamWorkloadIdentityPoolProvider_X509_TrustStore>>,
    ) -> Self {
        self.trust_store = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamWorkloadIdentityPoolProvider_X509 {
    type O = terrars::BlockAssignable<DataIamWorkloadIdentityPoolProvider_X509>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamWorkloadIdentityPoolProvider_X509 {}

impl BuildDataIamWorkloadIdentityPoolProvider_X509 {
    pub fn build(self) -> DataIamWorkloadIdentityPoolProvider_X509 {
        DataIamWorkloadIdentityPoolProvider_X509 { trust_store: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataIamWorkloadIdentityPoolProvider_X509Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamWorkloadIdentityPoolProvider_X509Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIamWorkloadIdentityPoolProvider_X509Ref {
        DataIamWorkloadIdentityPoolProvider_X509Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamWorkloadIdentityPoolProvider_X509Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `trust_store` after provisioning.\n"]
    pub fn trust_store(&self) -> terrars::ListRef<DataIamWorkloadIdentityPoolProvider_X509_TrustStoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust_store", self.base))
    }
}
