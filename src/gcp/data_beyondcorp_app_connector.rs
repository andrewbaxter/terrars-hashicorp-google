use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataBeyondcorpAppConnectorData {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataBeyondcorpAppConnector_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBeyondcorpAppConnectorData>,
}

#[derive(Clone)]
pub struct DataBeyondcorpAppConnector(std::rc::Rc<DataBeyondcorpAppConnector_>);

impl DataBeyondcorpAppConnector {
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

    #[doc = "Set the field `region`.\nThe region of the AppConnector."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nAn arbitrary user-provided name for the AppConnector."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nID of the AppConnector."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `principal_info` after provisioning.\nPrincipal information about the Identity of the AppConnector."]
    pub fn principal_info(&self) -> terrars::ListRef<DataBeyondcorpAppConnectorPrincipalInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the AppConnector."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nRepresents the different states of a AppConnector."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }
}

impl terrars::Referable for DataBeyondcorpAppConnector {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBeyondcorpAppConnector { }

impl terrars::ToListMappable for DataBeyondcorpAppConnector {
    type O = terrars::ListRef<DataBeyondcorpAppConnectorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBeyondcorpAppConnector_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_beyondcorp_app_connector".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBeyondcorpAppConnector {
    pub tf_id: std::string::String,
    #[doc = "ID of the AppConnector."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataBeyondcorpAppConnector {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBeyondcorpAppConnector {
        let out = DataBeyondcorpAppConnector(std::rc::Rc::new(DataBeyondcorpAppConnector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBeyondcorpAppConnectorData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBeyondcorpAppConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBeyondcorpAppConnectorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBeyondcorpAppConnectorRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nAn arbitrary user-provided name for the AppConnector."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nID of the AppConnector."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `principal_info` after provisioning.\nPrincipal information about the Identity of the AppConnector."]
    pub fn principal_info(&self) -> terrars::ListRef<DataBeyondcorpAppConnectorPrincipalInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the AppConnector."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nRepresents the different states of a AppConnector."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl {
    type O = terrars::BlockAssignable<DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl {}

impl BuildDataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl {
    pub fn build(self) -> DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl {
        DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl { email: core::default::Default::default() }
    }
}

pub struct DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountElRef {
        DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBeyondcorpAppConnectorPrincipalInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::ListField<DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl>>,
}

impl DataBeyondcorpAppConnectorPrincipalInfoEl {
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl>>,
    ) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBeyondcorpAppConnectorPrincipalInfoEl {
    type O = terrars::BlockAssignable<DataBeyondcorpAppConnectorPrincipalInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBeyondcorpAppConnectorPrincipalInfoEl {}

impl BuildDataBeyondcorpAppConnectorPrincipalInfoEl {
    pub fn build(self) -> DataBeyondcorpAppConnectorPrincipalInfoEl {
        DataBeyondcorpAppConnectorPrincipalInfoEl { service_account: core::default::Default::default() }
    }
}

pub struct DataBeyondcorpAppConnectorPrincipalInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBeyondcorpAppConnectorPrincipalInfoElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBeyondcorpAppConnectorPrincipalInfoElRef {
        DataBeyondcorpAppConnectorPrincipalInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBeyondcorpAppConnectorPrincipalInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> terrars::ListRef<DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account", self.base))
    }
}
