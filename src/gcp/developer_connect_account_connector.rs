#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DeveloperConnectAccountConnectorData_ {
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
    account_connector_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_oauth_config: std::option::Option<std::vec::Vec<DeveloperConnectAccountConnector_CustomOauthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider_oauth_config: std::option::Option<std::vec::Vec<DeveloperConnectAccountConnector_ProviderOauthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_config: std::option::Option<std::vec::Vec<DeveloperConnectAccountConnector_ProxyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DeveloperConnectAccountConnector_Timeouts>,
    dynamic: DeveloperConnectAccountConnectorDynamic,
}

struct DeveloperConnectAccountConnector_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DeveloperConnectAccountConnectorData_>,
}

#[derive(Clone)]
pub struct DeveloperConnectAccountConnector(std::rc::Rc<DeveloperConnectAccountConnector_>);

impl DeveloperConnectAccountConnector {
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
        "Set the field `annotations`.\nAllows users to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `etag`.\nThis checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding."]
    pub fn set_etag(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().etag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_oauth_config`.\n"]
    pub fn set_custom_oauth_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DeveloperConnectAccountConnector_CustomOauthConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_oauth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_oauth_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `provider_oauth_config`.\n"]
    pub fn set_provider_oauth_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DeveloperConnectAccountConnector_ProviderOauthConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().provider_oauth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.provider_oauth_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proxy_config`.\n"]
    pub fn set_proxy_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DeveloperConnectAccountConnector_ProxyConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().proxy_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.proxy_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DeveloperConnectAccountConnector_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_connector_id` after provisioning.\nThe ID to use for the AccountConnector, which will become the final\ncomponent of the AccountConnector's resource name. Its format should adhere\nto https://google.aip.dev/122#resource-id-segments Names must be unique\nper-project per-location."]
    pub fn account_connector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_connector_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAllows users to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the accountConnector was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the accountConnector, in the format\n'projects/{project}/locations/{location}/accountConnectors/{account_connector_id}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oauth_start_uri` after provisioning.\nStart OAuth flow by clicking on this URL."]
    pub fn oauth_start_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth_start_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the accountConnector was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_oauth_config` after provisioning.\n"]
    pub fn custom_oauth_config(&self) -> terrars::ListRef<DeveloperConnectAccountConnector_CustomOauthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_oauth_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provider_oauth_config` after provisioning.\n"]
    pub fn provider_oauth_config(&self) -> terrars::ListRef<DeveloperConnectAccountConnector_ProviderOauthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provider_oauth_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_config` after provisioning.\n"]
    pub fn proxy_config(&self) -> terrars::ListRef<DeveloperConnectAccountConnector_ProxyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DeveloperConnectAccountConnector_TimeoutsRef {
        DeveloperConnectAccountConnector_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DeveloperConnectAccountConnector {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DeveloperConnectAccountConnector { }

impl terrars::ToListMappable for DeveloperConnectAccountConnector {
    type O = terrars::ListRef<DeveloperConnectAccountConnectorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DeveloperConnectAccountConnector_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_developer_connect_account_connector".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDeveloperConnectAccountConnector {
    pub tf_id: std::string::String,
    #[doc =
        "The ID to use for the AccountConnector, which will become the final\ncomponent of the AccountConnector's resource name. Its format should adhere\nto https://google.aip.dev/122#resource-id-segments Names must be unique\nper-project per-location."]
    pub account_connector_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectAccountConnector {
    pub fn build(self, stack: &mut terrars::Stack) -> DeveloperConnectAccountConnector {
        let out = DeveloperConnectAccountConnector(std::rc::Rc::new(DeveloperConnectAccountConnector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DeveloperConnectAccountConnectorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_connector_id: self.account_connector_id,
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                etag: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                custom_oauth_config: core::default::Default::default(),
                provider_oauth_config: core::default::Default::default(),
                proxy_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DeveloperConnectAccountConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectAccountConnectorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DeveloperConnectAccountConnectorRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_connector_id` after provisioning.\nThe ID to use for the AccountConnector, which will become the final\ncomponent of the AccountConnector's resource name. Its format should adhere\nto https://google.aip.dev/122#resource-id-segments Names must be unique\nper-project per-location."]
    pub fn account_connector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_connector_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAllows users to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the accountConnector was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the accountConnector, in the format\n'projects/{project}/locations/{location}/accountConnectors/{account_connector_id}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oauth_start_uri` after provisioning.\nStart OAuth flow by clicking on this URL."]
    pub fn oauth_start_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth_start_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the accountConnector was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_oauth_config` after provisioning.\n"]
    pub fn custom_oauth_config(&self) -> terrars::ListRef<DeveloperConnectAccountConnector_CustomOauthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_oauth_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provider_oauth_config` after provisioning.\n"]
    pub fn provider_oauth_config(&self) -> terrars::ListRef<DeveloperConnectAccountConnector_ProviderOauthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provider_oauth_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_config` after provisioning.\n"]
    pub fn proxy_config(&self) -> terrars::ListRef<DeveloperConnectAccountConnector_ProxyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DeveloperConnectAccountConnector_TimeoutsRef {
        DeveloperConnectAccountConnector_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DeveloperConnectAccountConnectorDynamic {
    custom_oauth_config: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectAccountConnector_CustomOauthConfig>>,
    provider_oauth_config: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectAccountConnector_ProviderOauthConfig>>,
    proxy_config: std::option::Option<terrars::DynamicBlock<DeveloperConnectAccountConnector_ProxyConfig>>,
}

#[derive(Serialize)]
pub struct DeveloperConnectAccountConnector_CustomOauthConfig_ServiceDirectoryConfig {
    service: terrars::PrimField<std::string::String>,
}

impl DeveloperConnectAccountConnector_CustomOauthConfig_ServiceDirectoryConfig { }

impl terrars::ToListMappable for DeveloperConnectAccountConnector_CustomOauthConfig_ServiceDirectoryConfig {
    type O = terrars::BlockAssignable<DeveloperConnectAccountConnector_CustomOauthConfig_ServiceDirectoryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectAccountConnector_CustomOauthConfig_ServiceDirectoryConfig {
    #[doc =
        "The Service Directory service name.\nFormat:\nprojects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectAccountConnector_CustomOauthConfig_ServiceDirectoryConfig {
    pub fn build(self) -> DeveloperConnectAccountConnector_CustomOauthConfig_ServiceDirectoryConfig {
        DeveloperConnectAccountConnector_CustomOauthConfig_ServiceDirectoryConfig { service: self.service }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectAccountConnector_CustomOauthConfig_ServiceDirectoryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectAccountConnector_CustomOauthConfig_ServiceDirectoryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectAccountConnector_CustomOauthConfig_ServiceDirectoryConfigRef {
        DeveloperConnectAccountConnector_CustomOauthConfig_ServiceDirectoryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectAccountConnector_CustomOauthConfig_ServiceDirectoryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe Service Directory service name.\nFormat:\nprojects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize, Default)]
struct DeveloperConnectAccountConnector_CustomOauthConfigDynamic {
    service_directory_config: std
    ::option
    ::Option<terrars::DynamicBlock<DeveloperConnectAccountConnector_CustomOauthConfig_ServiceDirectoryConfig>>,
}

#[derive(Serialize)]
pub struct DeveloperConnectAccountConnector_CustomOauthConfig {
    auth_uri: terrars::PrimField<std::string::String>,
    client_id: terrars::PrimField<std::string::String>,
    client_secret: terrars::PrimField<std::string::String>,
    host_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pkce_disabled: std::option::Option<terrars::PrimField<bool>>,
    scm_provider: terrars::PrimField<std::string::String>,
    scopes: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_ca_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    token_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_config: std
    ::option
    ::Option<std::vec::Vec<DeveloperConnectAccountConnector_CustomOauthConfig_ServiceDirectoryConfig>>,
    dynamic: DeveloperConnectAccountConnector_CustomOauthConfigDynamic,
}

impl DeveloperConnectAccountConnector_CustomOauthConfig {
    #[doc = "Set the field `pkce_disabled`.\nDisable PKCE for this OAuth config. PKCE is enabled by default."]
    pub fn set_pkce_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.pkce_disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_ca_certificate`.\nSSL certificate to use for requests to a private service."]
    pub fn set_ssl_ca_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_ca_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DeveloperConnectAccountConnector_CustomOauthConfig_ServiceDirectoryConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_directory_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_directory_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectAccountConnector_CustomOauthConfig {
    type O = terrars::BlockAssignable<DeveloperConnectAccountConnector_CustomOauthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectAccountConnector_CustomOauthConfig {
    #[doc = "The OAuth2 authrization server URL."]
    pub auth_uri: terrars::PrimField<std::string::String>,
    #[doc = "The client ID of the OAuth application."]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Input only. The client secret of the OAuth application.\nIt will be provided as plain text, but encrypted and stored in developer\nconnect. As INPUT_ONLY field, it will not be included in the output."]
    pub client_secret: terrars::PrimField<std::string::String>,
    #[doc = "The host URI of the OAuth application."]
    pub host_uri: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of the SCM provider.\nPossible values:\nSCM_PROVIDER_UNKNOWN\nGITHUB_ENTERPRISE\nGITLAB_ENTERPRISE\nBITBUCKET_DATA_CENTER"]
    pub scm_provider: terrars::PrimField<std::string::String>,
    #[doc = "The scopes to be requested during OAuth."]
    pub scopes: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "The OAuth2 token request URL."]
    pub token_uri: terrars::PrimField<std::string::String>,
}

impl BuildDeveloperConnectAccountConnector_CustomOauthConfig {
    pub fn build(self) -> DeveloperConnectAccountConnector_CustomOauthConfig {
        DeveloperConnectAccountConnector_CustomOauthConfig {
            auth_uri: self.auth_uri,
            client_id: self.client_id,
            client_secret: self.client_secret,
            host_uri: self.host_uri,
            pkce_disabled: core::default::Default::default(),
            scm_provider: self.scm_provider,
            scopes: self.scopes,
            ssl_ca_certificate: core::default::Default::default(),
            token_uri: self.token_uri,
            service_directory_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectAccountConnector_CustomOauthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectAccountConnector_CustomOauthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectAccountConnector_CustomOauthConfigRef {
        DeveloperConnectAccountConnector_CustomOauthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectAccountConnector_CustomOauthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_uri` after provisioning.\nThe OAuth2 authrization server URL."]
    pub fn auth_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nThe client ID of the OAuth application."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nInput only. The client secret of the OAuth application.\nIt will be provided as plain text, but encrypted and stored in developer\nconnect. As INPUT_ONLY field, it will not be included in the output."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host_uri` after provisioning.\nThe host URI of the OAuth application."]
    pub fn host_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pkce_disabled` after provisioning.\nDisable PKCE for this OAuth config. PKCE is enabled by default."]
    pub fn pkce_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pkce_disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scm_provider` after provisioning.\nThe type of the SCM provider.\nPossible values:\nSCM_PROVIDER_UNKNOWN\nGITHUB_ENTERPRISE\nGITLAB_ENTERPRISE\nBITBUCKET_DATA_CENTER"]
    pub fn scm_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scm_provider", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scopes` after provisioning.\nThe scopes to be requested during OAuth."]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server_version` after provisioning.\nSCM server version installed at the host URI."]
    pub fn server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_ca_certificate` after provisioning.\nSSL certificate to use for requests to a private service."]
    pub fn ssl_ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `token_uri` after provisioning.\nThe OAuth2 token request URL."]
    pub fn token_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> terrars::ListRef<DeveloperConnectAccountConnector_CustomOauthConfig_ServiceDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectAccountConnector_ProviderOauthConfig {
    scopes: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_provider_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DeveloperConnectAccountConnector_ProviderOauthConfig {
    #[doc =
        "Set the field `system_provider_id`.\nPossible values:\nGITHUB\nGITLAB\nGOOGLE\nSENTRY\nROVO\nNEW_RELIC\nDATASTAX\nDYNATRACE"]
    pub fn set_system_provider_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.system_provider_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectAccountConnector_ProviderOauthConfig {
    type O = terrars::BlockAssignable<DeveloperConnectAccountConnector_ProviderOauthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectAccountConnector_ProviderOauthConfig {
    #[doc =
        "User selected scopes to apply to the Oauth config\nIn the event of changing scopes, user records under AccountConnector will\nbe deleted and users will re-auth again."]
    pub scopes: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDeveloperConnectAccountConnector_ProviderOauthConfig {
    pub fn build(self) -> DeveloperConnectAccountConnector_ProviderOauthConfig {
        DeveloperConnectAccountConnector_ProviderOauthConfig {
            scopes: self.scopes,
            system_provider_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectAccountConnector_ProviderOauthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectAccountConnector_ProviderOauthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectAccountConnector_ProviderOauthConfigRef {
        DeveloperConnectAccountConnector_ProviderOauthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectAccountConnector_ProviderOauthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `scopes` after provisioning.\nUser selected scopes to apply to the Oauth config\nIn the event of changing scopes, user records under AccountConnector will\nbe deleted and users will re-auth again."]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `system_provider_id` after provisioning.\nPossible values:\nGITHUB\nGITLAB\nGOOGLE\nSENTRY\nROVO\nNEW_RELIC\nDATASTAX\nDYNATRACE"]
    pub fn system_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectAccountConnector_ProxyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DeveloperConnectAccountConnector_ProxyConfig {
    #[doc =
        "Set the field `enabled`.\nSetting this to true allows the git and http proxies to perform actions on\nbehalf of the user configured under the account connector."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DeveloperConnectAccountConnector_ProxyConfig {
    type O = terrars::BlockAssignable<DeveloperConnectAccountConnector_ProxyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectAccountConnector_ProxyConfig {}

impl BuildDeveloperConnectAccountConnector_ProxyConfig {
    pub fn build(self) -> DeveloperConnectAccountConnector_ProxyConfig {
        DeveloperConnectAccountConnector_ProxyConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectAccountConnector_ProxyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectAccountConnector_ProxyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeveloperConnectAccountConnector_ProxyConfigRef {
        DeveloperConnectAccountConnector_ProxyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectAccountConnector_ProxyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nSetting this to true allows the git and http proxies to perform actions on\nbehalf of the user configured under the account connector."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DeveloperConnectAccountConnector_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DeveloperConnectAccountConnector_Timeouts {
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

impl terrars::ToListMappable for DeveloperConnectAccountConnector_Timeouts {
    type O = terrars::BlockAssignable<DeveloperConnectAccountConnector_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeveloperConnectAccountConnector_Timeouts {}

impl BuildDeveloperConnectAccountConnector_Timeouts {
    pub fn build(self) -> DeveloperConnectAccountConnector_Timeouts {
        DeveloperConnectAccountConnector_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DeveloperConnectAccountConnector_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeveloperConnectAccountConnector_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DeveloperConnectAccountConnector_TimeoutsRef {
        DeveloperConnectAccountConnector_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeveloperConnectAccountConnector_TimeoutsRef {
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
