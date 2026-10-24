use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CertificateManagerCertificateIssuanceConfigData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    key_algorithm: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    lifetime: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    rotation_window_percentage: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_authority_config: std
    ::option
    ::Option<std::vec::Vec<CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CertificateManagerCertificateIssuanceConfigTimeoutsEl>,
    dynamic: CertificateManagerCertificateIssuanceConfigDynamic,
}

struct CertificateManagerCertificateIssuanceConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CertificateManagerCertificateIssuanceConfigData>,
}

#[derive(Clone)]
pub struct CertificateManagerCertificateIssuanceConfig(std::rc::Rc<CertificateManagerCertificateIssuanceConfig_>);

impl CertificateManagerCertificateIssuanceConfig {
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

    #[doc = "Set the field `description`.\nOne or more paragraphs of text description of a CertificateIssuanceConfig."]
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
        "Set the field `labels`.\n'Set of label tags associated with the CertificateIssuanceConfig resource.\n An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nThe Certificate Manager location. If not specified, \"global\" is used."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_authority_config`.\n"]
    pub fn set_certificate_authority_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().certificate_authority_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.certificate_authority_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CertificateManagerCertificateIssuanceConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe creation timestamp of a CertificateIssuanceConfig. Timestamp is in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds with up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOne or more paragraphs of text description of a CertificateIssuanceConfig."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key_algorithm` after provisioning.\nKey algorithm to use when generating the private key. Possible values: [\"RSA_2048\", \"ECDSA_P256\"]"]
    pub fn key_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_algorithm", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\n'Set of label tags associated with the CertificateIssuanceConfig resource.\n An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lifetime` after provisioning.\nLifetime of issued certificates. A duration in seconds with up to nine fractional digits, ending with 's'.\nExample: \"1814400s\". Valid values are from 21 days (1814400s) to 30 days (2592000s)"]
    pub fn lifetime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifetime", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe Certificate Manager location. If not specified, \"global\" is used."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the certificate issuance config.\nCertificateIssuanceConfig names must be unique globally."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rotation_window_percentage` after provisioning.\nIt specifies the percentage of elapsed time of the certificate lifetime to wait before renewing the certificate.\nMust be a number between 1-99, inclusive.\nYou must set the rotation window percentage in relation to the certificate lifetime so that certificate renewal occurs at least 7 days after\nthe certificate has been issued and at least 7 days before it expires."]
    pub fn rotation_window_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rotation_window_percentage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe last update timestamp of a CertificateIssuanceConfig. Timestamp is in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds with up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_authority_config` after provisioning.\n"]
    pub fn certificate_authority_config(
        &self,
    ) -> terrars::ListRef<CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_authority_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CertificateManagerCertificateIssuanceConfigTimeoutsElRef {
        CertificateManagerCertificateIssuanceConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for CertificateManagerCertificateIssuanceConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CertificateManagerCertificateIssuanceConfig { }

impl terrars::ToListMappable for CertificateManagerCertificateIssuanceConfig {
    type O = terrars::ListRef<CertificateManagerCertificateIssuanceConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CertificateManagerCertificateIssuanceConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_certificate_manager_certificate_issuance_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCertificateManagerCertificateIssuanceConfig {
    pub tf_id: std::string::String,
    #[doc = "Key algorithm to use when generating the private key. Possible values: [\"RSA_2048\", \"ECDSA_P256\"]"]
    pub key_algorithm: terrars::PrimField<std::string::String>,
    #[doc =
        "Lifetime of issued certificates. A duration in seconds with up to nine fractional digits, ending with 's'.\nExample: \"1814400s\". Valid values are from 21 days (1814400s) to 30 days (2592000s)"]
    pub lifetime: terrars::PrimField<std::string::String>,
    #[doc =
        "A user-defined name of the certificate issuance config.\nCertificateIssuanceConfig names must be unique globally."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "It specifies the percentage of elapsed time of the certificate lifetime to wait before renewing the certificate.\nMust be a number between 1-99, inclusive.\nYou must set the rotation window percentage in relation to the certificate lifetime so that certificate renewal occurs at least 7 days after\nthe certificate has been issued and at least 7 days before it expires."]
    pub rotation_window_percentage: terrars::PrimField<f64>,
}

impl BuildCertificateManagerCertificateIssuanceConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> CertificateManagerCertificateIssuanceConfig {
        let out =
            CertificateManagerCertificateIssuanceConfig(
                std::rc::Rc::new(CertificateManagerCertificateIssuanceConfig_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(CertificateManagerCertificateIssuanceConfigData {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        provisioner: core::default::Default::default(),
                        description: core::default::Default::default(),
                        id: core::default::Default::default(),
                        key_algorithm: self.key_algorithm,
                        labels: core::default::Default::default(),
                        lifetime: self.lifetime,
                        location: core::default::Default::default(),
                        name: self.name,
                        project: core::default::Default::default(),
                        rotation_window_percentage: self.rotation_window_percentage,
                        certificate_authority_config: core::default::Default::default(),
                        timeouts: core::default::Default::default(),
                        dynamic: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CertificateManagerCertificateIssuanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerCertificateIssuanceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CertificateManagerCertificateIssuanceConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe creation timestamp of a CertificateIssuanceConfig. Timestamp is in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds with up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOne or more paragraphs of text description of a CertificateIssuanceConfig."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key_algorithm` after provisioning.\nKey algorithm to use when generating the private key. Possible values: [\"RSA_2048\", \"ECDSA_P256\"]"]
    pub fn key_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_algorithm", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\n'Set of label tags associated with the CertificateIssuanceConfig resource.\n An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lifetime` after provisioning.\nLifetime of issued certificates. A duration in seconds with up to nine fractional digits, ending with 's'.\nExample: \"1814400s\". Valid values are from 21 days (1814400s) to 30 days (2592000s)"]
    pub fn lifetime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifetime", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe Certificate Manager location. If not specified, \"global\" is used."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the certificate issuance config.\nCertificateIssuanceConfig names must be unique globally."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rotation_window_percentage` after provisioning.\nIt specifies the percentage of elapsed time of the certificate lifetime to wait before renewing the certificate.\nMust be a number between 1-99, inclusive.\nYou must set the rotation window percentage in relation to the certificate lifetime so that certificate renewal occurs at least 7 days after\nthe certificate has been issued and at least 7 days before it expires."]
    pub fn rotation_window_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rotation_window_percentage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe last update timestamp of a CertificateIssuanceConfig. Timestamp is in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds with up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_authority_config` after provisioning.\n"]
    pub fn certificate_authority_config(
        &self,
    ) -> terrars::ListRef<CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_authority_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CertificateManagerCertificateIssuanceConfigTimeoutsElRef {
        CertificateManagerCertificateIssuanceConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl {
    ca_pool: terrars::PrimField<std::string::String>,
}

impl CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl { }

impl terrars
::ToListMappable for CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl {
    #[doc =
        "A CA pool resource used to issue a certificate.\nThe CA pool string has a relative resource path following the form\n\"projects/{project}/locations/{location}/caPools/{caPool}\"."]
    pub ca_pool: terrars::PrimField<std::string::String>,
}

impl BuildCertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl {
    pub fn build(
        self,
    ) -> CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl {
        CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl {
            ca_pool: self.ca_pool,
        }
    }
}

pub struct CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigElRef {
        CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ca_pool` after provisioning.\nA CA pool resource used to issue a certificate.\nThe CA pool string has a relative resource path following the form\n\"projects/{project}/locations/{location}/caPools/{caPool}\"."]
    pub fn ca_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_pool", self.base))
    }
}

#[derive(Serialize, Default)]
struct CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElDynamic {
    certificate_authority_service_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_authority_service_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl,
        >,
    >,
    dynamic: CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElDynamic,
}

impl CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl {
    #[doc = "Set the field `certificate_authority_service_config`.\n"]
    pub fn set_certificate_authority_service_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.certificate_authority_service_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.certificate_authority_service_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl {
    type O = terrars::BlockAssignable<CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl {}

impl BuildCertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl {
    pub fn build(self) -> CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl {
        CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl {
            certificate_authority_service_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElRef {
        CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_service_config` after provisioning.\n"]
    pub fn certificate_authority_service_config(
        &self,
    ) -> terrars
    ::ListRef<
        CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigElCertificateAuthorityServiceConfigElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_authority_service_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CertificateManagerCertificateIssuanceConfigTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CertificateManagerCertificateIssuanceConfigTimeoutsEl {
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
}

impl terrars::ToListMappable for CertificateManagerCertificateIssuanceConfigTimeoutsEl {
    type O = terrars::BlockAssignable<CertificateManagerCertificateIssuanceConfigTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCertificateManagerCertificateIssuanceConfigTimeoutsEl {}

impl BuildCertificateManagerCertificateIssuanceConfigTimeoutsEl {
    pub fn build(self) -> CertificateManagerCertificateIssuanceConfigTimeoutsEl {
        CertificateManagerCertificateIssuanceConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct CertificateManagerCertificateIssuanceConfigTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CertificateManagerCertificateIssuanceConfigTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CertificateManagerCertificateIssuanceConfigTimeoutsElRef {
        CertificateManagerCertificateIssuanceConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CertificateManagerCertificateIssuanceConfigTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct CertificateManagerCertificateIssuanceConfigDynamic {
    certificate_authority_config: std
    ::option
    ::Option<terrars::DynamicBlock<CertificateManagerCertificateIssuanceConfigCertificateAuthorityConfigEl>>,
}
