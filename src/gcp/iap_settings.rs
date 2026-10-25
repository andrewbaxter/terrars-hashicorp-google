#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IapSettingsData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_settings: std::option::Option<std::vec::Vec<IapSettings_AccessSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_settings: std::option::Option<std::vec::Vec<IapSettings_ApplicationSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IapSettings_Timeouts>,
    dynamic: IapSettingsDynamic,
}

struct IapSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IapSettingsData_>,
}

#[derive(Clone)]
pub struct IapSettings(std::rc::Rc<IapSettings_>);

impl IapSettings {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_settings`.\n"]
    pub fn set_access_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IapSettings_AccessSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `application_settings`.\n"]
    pub fn set_application_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IapSettings_ApplicationSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().application_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.application_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<IapSettings_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the IAP protected resource. Name can have below resources:\n* organizations/{organization_id}\n* folders/{folder_id}\n* projects/{project_id}\n* projects/{project_id}/iap_web\n* projects/{project_id}/iap_web/compute\n* projects/{project_id}/iap_web/compute-{region}\n* projects/{project_id}/iap_web/compute/services/{service_id}\n* projects/{project_id}/iap_web/compute-{region}/services/{service_id}\n* projects/{project_id}/iap_web/forwarding_rule\n* projects/{project_id}/iap_web/forwarding_rule-{region}\n* projects/{project_id}/iap_web/forwarding_rule/services/{service_id}\n* projects/{project_id}/iap_web/forwarding_rule-{region}/services/{service_id}\n* projects/{project_id}/iap_web/appengine-{app_id}\n* projects/{project_id}/iap_web/appengine-{app_id}/services/{service_id}\n* projects/{project_id}/iap_web/appengine-{app_id}/services/{service_id}/version/{version_id}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_settings` after provisioning.\n"]
    pub fn access_settings(&self) -> terrars::ListRef<IapSettings_AccessSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_settings` after provisioning.\n"]
    pub fn application_settings(&self) -> terrars::ListRef<IapSettings_ApplicationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IapSettings_TimeoutsRef {
        IapSettings_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for IapSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IapSettings { }

impl terrars::ToListMappable for IapSettings {
    type O = terrars::ListRef<IapSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IapSettings_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_iap_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIapSettings {
    pub tf_id: std::string::String,
    #[doc =
        "The resource name of the IAP protected resource. Name can have below resources:\n* organizations/{organization_id}\n* folders/{folder_id}\n* projects/{project_id}\n* projects/{project_id}/iap_web\n* projects/{project_id}/iap_web/compute\n* projects/{project_id}/iap_web/compute-{region}\n* projects/{project_id}/iap_web/compute/services/{service_id}\n* projects/{project_id}/iap_web/compute-{region}/services/{service_id}\n* projects/{project_id}/iap_web/forwarding_rule\n* projects/{project_id}/iap_web/forwarding_rule-{region}\n* projects/{project_id}/iap_web/forwarding_rule/services/{service_id}\n* projects/{project_id}/iap_web/forwarding_rule-{region}/services/{service_id}\n* projects/{project_id}/iap_web/appengine-{app_id}\n* projects/{project_id}/iap_web/appengine-{app_id}/services/{service_id}\n* projects/{project_id}/iap_web/appengine-{app_id}/services/{service_id}/version/{version_id}"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildIapSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> IapSettings {
        let out = IapSettings(std::rc::Rc::new(IapSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IapSettingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                access_settings: core::default::Default::default(),
                application_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IapSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IapSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IapSettingsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the IAP protected resource. Name can have below resources:\n* organizations/{organization_id}\n* folders/{folder_id}\n* projects/{project_id}\n* projects/{project_id}/iap_web\n* projects/{project_id}/iap_web/compute\n* projects/{project_id}/iap_web/compute-{region}\n* projects/{project_id}/iap_web/compute/services/{service_id}\n* projects/{project_id}/iap_web/compute-{region}/services/{service_id}\n* projects/{project_id}/iap_web/forwarding_rule\n* projects/{project_id}/iap_web/forwarding_rule-{region}\n* projects/{project_id}/iap_web/forwarding_rule/services/{service_id}\n* projects/{project_id}/iap_web/forwarding_rule-{region}/services/{service_id}\n* projects/{project_id}/iap_web/appengine-{app_id}\n* projects/{project_id}/iap_web/appengine-{app_id}/services/{service_id}\n* projects/{project_id}/iap_web/appengine-{app_id}/services/{service_id}/version/{version_id}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_settings` after provisioning.\n"]
    pub fn access_settings(&self) -> terrars::ListRef<IapSettings_AccessSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_settings` after provisioning.\n"]
    pub fn application_settings(&self) -> terrars::ListRef<IapSettings_ApplicationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IapSettings_TimeoutsRef {
        IapSettings_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct IapSettingsDynamic {
    access_settings: std::option::Option<terrars::DynamicBlock<IapSettings_AccessSettings>>,
    application_settings: std::option::Option<terrars::DynamicBlock<IapSettings_ApplicationSettings>>,
}

#[derive(Serialize)]
pub struct IapSettings_AccessSettings_AllowedDomainsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable: std::option::Option<terrars::PrimField<bool>>,
}

impl IapSettings_AccessSettings_AllowedDomainsSettings {
    #[doc = "Set the field `domains`.\nList of trusted domains."]
    pub fn set_domains(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.domains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable`.\nConfiguration for customers to opt in for the feature."]
    pub fn set_enable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IapSettings_AccessSettings_AllowedDomainsSettings {
    type O = terrars::BlockAssignable<IapSettings_AccessSettings_AllowedDomainsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIapSettings_AccessSettings_AllowedDomainsSettings {}

impl BuildIapSettings_AccessSettings_AllowedDomainsSettings {
    pub fn build(self) -> IapSettings_AccessSettings_AllowedDomainsSettings {
        IapSettings_AccessSettings_AllowedDomainsSettings {
            domains: core::default::Default::default(),
            enable: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IapSettings_AccessSettings_AllowedDomainsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IapSettings_AccessSettings_AllowedDomainsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IapSettings_AccessSettings_AllowedDomainsSettingsRef {
        IapSettings_AccessSettings_AllowedDomainsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IapSettings_AccessSettings_AllowedDomainsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domains` after provisioning.\nList of trusted domains."]
    pub fn domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable` after provisioning.\nConfiguration for customers to opt in for the feature."]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
}

#[derive(Serialize)]
pub struct IapSettings_AccessSettings_CorsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_http_options: std::option::Option<terrars::PrimField<bool>>,
}

impl IapSettings_AccessSettings_CorsSettings {
    #[doc =
        "Set the field `allow_http_options`.\nConfiguration to allow HTTP OPTIONS calls to skip authorization.\nIf undefined, IAP will not apply any special logic to OPTIONS requests."]
    pub fn set_allow_http_options(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_http_options = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IapSettings_AccessSettings_CorsSettings {
    type O = terrars::BlockAssignable<IapSettings_AccessSettings_CorsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIapSettings_AccessSettings_CorsSettings {}

impl BuildIapSettings_AccessSettings_CorsSettings {
    pub fn build(self) -> IapSettings_AccessSettings_CorsSettings {
        IapSettings_AccessSettings_CorsSettings { allow_http_options: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct IapSettings_AccessSettings_CorsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IapSettings_AccessSettings_CorsSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IapSettings_AccessSettings_CorsSettingsRef {
        IapSettings_AccessSettings_CorsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IapSettings_AccessSettings_CorsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_http_options` after provisioning.\nConfiguration to allow HTTP OPTIONS calls to skip authorization.\nIf undefined, IAP will not apply any special logic to OPTIONS requests."]
    pub fn allow_http_options(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_http_options", self.base))
    }
}

#[derive(Serialize)]
pub struct IapSettings_AccessSettings_GcipSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_page_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl IapSettings_AccessSettings_GcipSettings {
    #[doc =
        "Set the field `login_page_uri`.\nLogin page URI associated with the GCIP tenants. Typically, all resources within\nthe same project share the same login page, though it could be overridden at the\nsub resource level."]
    pub fn set_login_page_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.login_page_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tenant_ids`.\nGCIP tenant ids that are linked to the IAP resource. tenantIds could be a string\nbeginning with a number character to indicate authenticating with GCIP tenant flow,\nor in the format of _ to indicate authenticating with GCIP agent flow. If agent flow\nis used, tenantIds should only contain one single element, while for tenant flow,\ntenantIds can contain multiple elements."]
    pub fn set_tenant_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tenant_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IapSettings_AccessSettings_GcipSettings {
    type O = terrars::BlockAssignable<IapSettings_AccessSettings_GcipSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIapSettings_AccessSettings_GcipSettings {}

impl BuildIapSettings_AccessSettings_GcipSettings {
    pub fn build(self) -> IapSettings_AccessSettings_GcipSettings {
        IapSettings_AccessSettings_GcipSettings {
            login_page_uri: core::default::Default::default(),
            tenant_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IapSettings_AccessSettings_GcipSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IapSettings_AccessSettings_GcipSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IapSettings_AccessSettings_GcipSettingsRef {
        IapSettings_AccessSettings_GcipSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IapSettings_AccessSettings_GcipSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `login_page_uri` after provisioning.\nLogin page URI associated with the GCIP tenants. Typically, all resources within\nthe same project share the same login page, though it could be overridden at the\nsub resource level."]
    pub fn login_page_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.login_page_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tenant_ids` after provisioning.\nGCIP tenant ids that are linked to the IAP resource. tenantIds could be a string\nbeginning with a number character to indicate authenticating with GCIP tenant flow,\nor in the format of _ to indicate authenticating with GCIP agent flow. If agent flow\nis used, tenantIds should only contain one single element, while for tenant flow,\ntenantIds can contain multiple elements."]
    pub fn tenant_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tenant_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct IapSettings_AccessSettings_OauthSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_hint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    programmatic_clients: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl IapSettings_AccessSettings_OauthSettings {
    #[doc =
        "Set the field `client_id`.\nOAuth 2.0 client ID used in the OAuth flow to generate an access token. If this field is set, you can skip obtaining the OAuth credentials in this."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\nOAuth secret paired with client ID."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `login_hint`.\nDomain hint to send as hd=? parameter in OAuth request flow.\nEnables redirect to primary IDP by skipping Google's login screen.\n(https://developers.google.com/identity/protocols/OpenIDConnect#hd-param)\nNote: IAP does not verify that the id token's hd claim matches this value\nsince access behavior is managed by IAM policies.\n* loginHint setting is not a replacement for access control. Always enforce an appropriate access policy if you want to restrict access to users outside your domain."]
    pub fn set_login_hint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.login_hint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `programmatic_clients`.\nList of client ids allowed to use IAP programmatically."]
    pub fn set_programmatic_clients(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.programmatic_clients = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IapSettings_AccessSettings_OauthSettings {
    type O = terrars::BlockAssignable<IapSettings_AccessSettings_OauthSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIapSettings_AccessSettings_OauthSettings {}

impl BuildIapSettings_AccessSettings_OauthSettings {
    pub fn build(self) -> IapSettings_AccessSettings_OauthSettings {
        IapSettings_AccessSettings_OauthSettings {
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            login_hint: core::default::Default::default(),
            programmatic_clients: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IapSettings_AccessSettings_OauthSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IapSettings_AccessSettings_OauthSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IapSettings_AccessSettings_OauthSettingsRef {
        IapSettings_AccessSettings_OauthSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IapSettings_AccessSettings_OauthSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nOAuth 2.0 client ID used in the OAuth flow to generate an access token. If this field is set, you can skip obtaining the OAuth credentials in this."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nOAuth secret paired with client ID."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret_sha256` after provisioning.\nOAuth secret sha256 paired with client ID."]
    pub fn client_secret_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_sha256", self.base))
    }

    #[doc =
        "Get a reference to the value of field `login_hint` after provisioning.\nDomain hint to send as hd=? parameter in OAuth request flow.\nEnables redirect to primary IDP by skipping Google's login screen.\n(https://developers.google.com/identity/protocols/OpenIDConnect#hd-param)\nNote: IAP does not verify that the id token's hd claim matches this value\nsince access behavior is managed by IAM policies.\n* loginHint setting is not a replacement for access control. Always enforce an appropriate access policy if you want to restrict access to users outside your domain."]
    pub fn login_hint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.login_hint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `programmatic_clients` after provisioning.\nList of client ids allowed to use IAP programmatically."]
    pub fn programmatic_clients(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.programmatic_clients", self.base))
    }
}

#[derive(Serialize)]
pub struct IapSettings_AccessSettings_ReauthSettings {
    max_age: terrars::PrimField<std::string::String>,
    method: terrars::PrimField<std::string::String>,
    policy_type: terrars::PrimField<std::string::String>,
}

impl IapSettings_AccessSettings_ReauthSettings { }

impl terrars::ToListMappable for IapSettings_AccessSettings_ReauthSettings {
    type O = terrars::BlockAssignable<IapSettings_AccessSettings_ReauthSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIapSettings_AccessSettings_ReauthSettings {
    #[doc =
        "Reauth session lifetime, how long before a user has to reauthenticate again.\nA duration in seconds with up to nine fractional digits, ending with 's'.\nExample: \"3.5s\"."]
    pub max_age: terrars::PrimField<std::string::String>,
    #[doc =
        "Reauth method requested. The possible values are:\n\n* 'LOGIN': Prompts the user to log in again.\n* 'SECURE_KEY': User must use their secure key 2nd factor device.\n* 'ENROLLED_SECOND_FACTORS': User can use any enabled 2nd factor. Possible values: [\"LOGIN\", \"SECURE_KEY\", \"ENROLLED_SECOND_FACTORS\"]"]
    pub method: terrars::PrimField<std::string::String>,
    #[doc =
        "How IAP determines the effective policy in cases of hierarchical policies.\nPolicies are merged from higher in the hierarchy to lower in the hierarchy.\nThe possible values are:\n\n* 'MINIMUM': This policy acts as a minimum to other policies, lower in the hierarchy.\n\t\t   Effective policy may only be the same or stricter.\n* 'DEFAULT': This policy acts as a default if no other reauth policy is set. Possible values: [\"MINIMUM\", \"DEFAULT\"]"]
    pub policy_type: terrars::PrimField<std::string::String>,
}

impl BuildIapSettings_AccessSettings_ReauthSettings {
    pub fn build(self) -> IapSettings_AccessSettings_ReauthSettings {
        IapSettings_AccessSettings_ReauthSettings {
            max_age: self.max_age,
            method: self.method,
            policy_type: self.policy_type,
        }
    }
}

#[allow(dead_code)]
pub struct IapSettings_AccessSettings_ReauthSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IapSettings_AccessSettings_ReauthSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IapSettings_AccessSettings_ReauthSettingsRef {
        IapSettings_AccessSettings_ReauthSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IapSettings_AccessSettings_ReauthSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_age` after provisioning.\nReauth session lifetime, how long before a user has to reauthenticate again.\nA duration in seconds with up to nine fractional digits, ending with 's'.\nExample: \"3.5s\"."]
    pub fn max_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nReauth method requested. The possible values are:\n\n* 'LOGIN': Prompts the user to log in again.\n* 'SECURE_KEY': User must use their secure key 2nd factor device.\n* 'ENROLLED_SECOND_FACTORS': User can use any enabled 2nd factor. Possible values: [\"LOGIN\", \"SECURE_KEY\", \"ENROLLED_SECOND_FACTORS\"]"]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `policy_type` after provisioning.\nHow IAP determines the effective policy in cases of hierarchical policies.\nPolicies are merged from higher in the hierarchy to lower in the hierarchy.\nThe possible values are:\n\n* 'MINIMUM': This policy acts as a minimum to other policies, lower in the hierarchy.\n\t\t   Effective policy may only be the same or stricter.\n* 'DEFAULT': This policy acts as a default if no other reauth policy is set. Possible values: [\"MINIMUM\", \"DEFAULT\"]"]
    pub fn policy_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_type", self.base))
    }
}

#[derive(Serialize)]
pub struct IapSettings_AccessSettings_WorkforceIdentitySettings_Oauth2 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IapSettings_AccessSettings_WorkforceIdentitySettings_Oauth2 {
    #[doc =
        "Set the field `client_id`.\nThe OAuth 2.0 client ID registered in the workforce identity\nfederation OAuth 2.0 Server."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_secret`.\nInput only. The OAuth 2.0 client secret created while registering\nthe client ID."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IapSettings_AccessSettings_WorkforceIdentitySettings_Oauth2 {
    type O = terrars::BlockAssignable<IapSettings_AccessSettings_WorkforceIdentitySettings_Oauth2>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIapSettings_AccessSettings_WorkforceIdentitySettings_Oauth2 {}

impl BuildIapSettings_AccessSettings_WorkforceIdentitySettings_Oauth2 {
    pub fn build(self) -> IapSettings_AccessSettings_WorkforceIdentitySettings_Oauth2 {
        IapSettings_AccessSettings_WorkforceIdentitySettings_Oauth2 {
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IapSettings_AccessSettings_WorkforceIdentitySettings_Oauth2Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IapSettings_AccessSettings_WorkforceIdentitySettings_Oauth2Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IapSettings_AccessSettings_WorkforceIdentitySettings_Oauth2Ref {
        IapSettings_AccessSettings_WorkforceIdentitySettings_Oauth2Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IapSettings_AccessSettings_WorkforceIdentitySettings_Oauth2Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nThe OAuth 2.0 client ID registered in the workforce identity\nfederation OAuth 2.0 Server."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nInput only. The OAuth 2.0 client secret created while registering\nthe client ID."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret_sha256` after provisioning.\nOutput only. SHA256 hash value for the client secret. This field\nis returned by IAP when the settings are retrieved."]
    pub fn client_secret_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_sha256", self.base))
    }
}

#[derive(Serialize, Default)]
struct IapSettings_AccessSettings_WorkforceIdentitySettingsDynamic {
    oauth2: std::option::Option<terrars::DynamicBlock<IapSettings_AccessSettings_WorkforceIdentitySettings_Oauth2>>,
}

#[derive(Serialize)]
pub struct IapSettings_AccessSettings_WorkforceIdentitySettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workforce_pools: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2: std::option::Option<std::vec::Vec<IapSettings_AccessSettings_WorkforceIdentitySettings_Oauth2>>,
    dynamic: IapSettings_AccessSettings_WorkforceIdentitySettingsDynamic,
}

impl IapSettings_AccessSettings_WorkforceIdentitySettings {
    #[doc = "Set the field `workforce_pools`.\nThe workforce pool resources. Only one workforce pool is accepted."]
    pub fn set_workforce_pools(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.workforce_pools = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth2`.\n"]
    pub fn set_oauth2(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IapSettings_AccessSettings_WorkforceIdentitySettings_Oauth2>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth2 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IapSettings_AccessSettings_WorkforceIdentitySettings {
    type O = terrars::BlockAssignable<IapSettings_AccessSettings_WorkforceIdentitySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIapSettings_AccessSettings_WorkforceIdentitySettings {}

impl BuildIapSettings_AccessSettings_WorkforceIdentitySettings {
    pub fn build(self) -> IapSettings_AccessSettings_WorkforceIdentitySettings {
        IapSettings_AccessSettings_WorkforceIdentitySettings {
            workforce_pools: core::default::Default::default(),
            oauth2: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IapSettings_AccessSettings_WorkforceIdentitySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IapSettings_AccessSettings_WorkforceIdentitySettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IapSettings_AccessSettings_WorkforceIdentitySettingsRef {
        IapSettings_AccessSettings_WorkforceIdentitySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IapSettings_AccessSettings_WorkforceIdentitySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `workforce_pools` after provisioning.\nThe workforce pool resources. Only one workforce pool is accepted."]
    pub fn workforce_pools(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workforce_pools", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2` after provisioning.\n"]
    pub fn oauth2(&self) -> terrars::ListRef<IapSettings_AccessSettings_WorkforceIdentitySettings_Oauth2Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2", self.base))
    }
}

#[derive(Serialize, Default)]
struct IapSettings_AccessSettingsDynamic {
    allowed_domains_settings: std
    ::option
    ::Option<terrars::DynamicBlock<IapSettings_AccessSettings_AllowedDomainsSettings>>,
    cors_settings: std::option::Option<terrars::DynamicBlock<IapSettings_AccessSettings_CorsSettings>>,
    gcip_settings: std::option::Option<terrars::DynamicBlock<IapSettings_AccessSettings_GcipSettings>>,
    oauth_settings: std::option::Option<terrars::DynamicBlock<IapSettings_AccessSettings_OauthSettings>>,
    reauth_settings: std::option::Option<terrars::DynamicBlock<IapSettings_AccessSettings_ReauthSettings>>,
    workforce_identity_settings: std
    ::option
    ::Option<terrars::DynamicBlock<IapSettings_AccessSettings_WorkforceIdentitySettings>>,
}

#[derive(Serialize)]
pub struct IapSettings_AccessSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_sources: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_domains_settings: std::option::Option<std::vec::Vec<IapSettings_AccessSettings_AllowedDomainsSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors_settings: std::option::Option<std::vec::Vec<IapSettings_AccessSettings_CorsSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcip_settings: std::option::Option<std::vec::Vec<IapSettings_AccessSettings_GcipSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_settings: std::option::Option<std::vec::Vec<IapSettings_AccessSettings_OauthSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reauth_settings: std::option::Option<std::vec::Vec<IapSettings_AccessSettings_ReauthSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workforce_identity_settings: std::option::Option<std::vec::Vec<IapSettings_AccessSettings_WorkforceIdentitySettings>>,
    dynamic: IapSettings_AccessSettingsDynamic,
}

impl IapSettings_AccessSettings {
    #[doc =
        "Set the field `identity_sources`.\nIdentity sources that IAP can use to authenticate the end user. Only one identity source\ncan be configured. The possible values are:\n\n* 'WORKFORCE_IDENTITY_FEDERATION': Use external identities set up on Google Cloud Workforce\n  \t\t\t\t     Identity Federation. Possible values: [\"WORKFORCE_IDENTITY_FEDERATION\"]"]
    pub fn set_identity_sources(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.identity_sources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_domains_settings`.\n"]
    pub fn set_allowed_domains_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IapSettings_AccessSettings_AllowedDomainsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowed_domains_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_domains_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cors_settings`.\n"]
    pub fn set_cors_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IapSettings_AccessSettings_CorsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cors_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cors_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gcip_settings`.\n"]
    pub fn set_gcip_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IapSettings_AccessSettings_GcipSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcip_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcip_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth_settings`.\n"]
    pub fn set_oauth_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IapSettings_AccessSettings_OauthSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `reauth_settings`.\n"]
    pub fn set_reauth_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IapSettings_AccessSettings_ReauthSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.reauth_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.reauth_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workforce_identity_settings`.\n"]
    pub fn set_workforce_identity_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IapSettings_AccessSettings_WorkforceIdentitySettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workforce_identity_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workforce_identity_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IapSettings_AccessSettings {
    type O = terrars::BlockAssignable<IapSettings_AccessSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIapSettings_AccessSettings {}

impl BuildIapSettings_AccessSettings {
    pub fn build(self) -> IapSettings_AccessSettings {
        IapSettings_AccessSettings {
            identity_sources: core::default::Default::default(),
            allowed_domains_settings: core::default::Default::default(),
            cors_settings: core::default::Default::default(),
            gcip_settings: core::default::Default::default(),
            oauth_settings: core::default::Default::default(),
            reauth_settings: core::default::Default::default(),
            workforce_identity_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IapSettings_AccessSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IapSettings_AccessSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IapSettings_AccessSettingsRef {
        IapSettings_AccessSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IapSettings_AccessSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identity_sources` after provisioning.\nIdentity sources that IAP can use to authenticate the end user. Only one identity source\ncan be configured. The possible values are:\n\n* 'WORKFORCE_IDENTITY_FEDERATION': Use external identities set up on Google Cloud Workforce\n  \t\t\t\t     Identity Federation. Possible values: [\"WORKFORCE_IDENTITY_FEDERATION\"]"]
    pub fn identity_sources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_sources", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_domains_settings` after provisioning.\n"]
    pub fn allowed_domains_settings(&self) -> terrars::ListRef<IapSettings_AccessSettings_AllowedDomainsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_domains_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `cors_settings` after provisioning.\n"]
    pub fn cors_settings(&self) -> terrars::ListRef<IapSettings_AccessSettings_CorsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `gcip_settings` after provisioning.\n"]
    pub fn gcip_settings(&self) -> terrars::ListRef<IapSettings_AccessSettings_GcipSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcip_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_settings` after provisioning.\n"]
    pub fn oauth_settings(&self) -> terrars::ListRef<IapSettings_AccessSettings_OauthSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `reauth_settings` after provisioning.\n"]
    pub fn reauth_settings(&self) -> terrars::ListRef<IapSettings_AccessSettings_ReauthSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reauth_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `workforce_identity_settings` after provisioning.\n"]
    pub fn workforce_identity_settings(
        &self,
    ) -> terrars::ListRef<IapSettings_AccessSettings_WorkforceIdentitySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workforce_identity_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct IapSettings_ApplicationSettings_AccessDeniedPageSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_denied_page_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generate_troubleshooting_uri: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remediation_token_generation_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl IapSettings_ApplicationSettings_AccessDeniedPageSettings {
    #[doc = "Set the field `access_denied_page_uri`.\nThe URI to be redirected to when access is denied."]
    pub fn set_access_denied_page_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.access_denied_page_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `generate_troubleshooting_uri`.\nWhether to generate a troubleshooting URL on access denied events to this application."]
    pub fn set_generate_troubleshooting_uri(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.generate_troubleshooting_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `remediation_token_generation_enabled`.\nWhether to generate remediation token on access denied events to this application."]
    pub fn set_remediation_token_generation_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.remediation_token_generation_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IapSettings_ApplicationSettings_AccessDeniedPageSettings {
    type O = terrars::BlockAssignable<IapSettings_ApplicationSettings_AccessDeniedPageSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIapSettings_ApplicationSettings_AccessDeniedPageSettings {}

impl BuildIapSettings_ApplicationSettings_AccessDeniedPageSettings {
    pub fn build(self) -> IapSettings_ApplicationSettings_AccessDeniedPageSettings {
        IapSettings_ApplicationSettings_AccessDeniedPageSettings {
            access_denied_page_uri: core::default::Default::default(),
            generate_troubleshooting_uri: core::default::Default::default(),
            remediation_token_generation_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IapSettings_ApplicationSettings_AccessDeniedPageSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IapSettings_ApplicationSettings_AccessDeniedPageSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IapSettings_ApplicationSettings_AccessDeniedPageSettingsRef {
        IapSettings_ApplicationSettings_AccessDeniedPageSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IapSettings_ApplicationSettings_AccessDeniedPageSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_denied_page_uri` after provisioning.\nThe URI to be redirected to when access is denied."]
    pub fn access_denied_page_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_denied_page_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `generate_troubleshooting_uri` after provisioning.\nWhether to generate a troubleshooting URL on access denied events to this application."]
    pub fn generate_troubleshooting_uri(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generate_troubleshooting_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `remediation_token_generation_enabled` after provisioning.\nWhether to generate remediation token on access denied events to this application."]
    pub fn remediation_token_generation_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remediation_token_generation_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct IapSettings_ApplicationSettings_AttributePropagationSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_credentials: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl IapSettings_ApplicationSettings_AttributePropagationSettings {
    #[doc =
        "Set the field `enable`.\nWhether the provided attribute propagation settings should be evaluated on user requests.\nIf set to true, attributes returned from the expression will be propagated in the set output credentials."]
    pub fn set_enable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expression`.\nRaw string CEL expression. Must return a list of attributes. A maximum of 45 attributes can\nbe selected. Expressions can select different attribute types from attributes:\nattributes.saml_attributes, attributes.iap_attributes."]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_credentials`.\nWhich output credentials attributes selected by the CEL expression should be propagated in.\nAll attributes will be fully duplicated in each selected output credential.\nPossible values are:\n\n* 'HEADER': Propagate attributes in the headers with \"x-goog-iap-attr-\" prefix.\n* 'JWT': Propagate attributes in the JWT of the form:\n         \"additional_claims\": { \"my_attribute\": [\"value1\", \"value2\"] }\n* 'RCTOKEN': Propagate attributes in the RCToken of the form: \"\n             additional_claims\": { \"my_attribute\": [\"value1\", \"value2\"] } Possible values: [\"HEADER\", \"JWT\", \"RCTOKEN\"]"]
    pub fn set_output_credentials(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.output_credentials = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IapSettings_ApplicationSettings_AttributePropagationSettings {
    type O = terrars::BlockAssignable<IapSettings_ApplicationSettings_AttributePropagationSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIapSettings_ApplicationSettings_AttributePropagationSettings {}

impl BuildIapSettings_ApplicationSettings_AttributePropagationSettings {
    pub fn build(self) -> IapSettings_ApplicationSettings_AttributePropagationSettings {
        IapSettings_ApplicationSettings_AttributePropagationSettings {
            enable: core::default::Default::default(),
            expression: core::default::Default::default(),
            output_credentials: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IapSettings_ApplicationSettings_AttributePropagationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IapSettings_ApplicationSettings_AttributePropagationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IapSettings_ApplicationSettings_AttributePropagationSettingsRef {
        IapSettings_ApplicationSettings_AttributePropagationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IapSettings_ApplicationSettings_AttributePropagationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable` after provisioning.\nWhether the provided attribute propagation settings should be evaluated on user requests.\nIf set to true, attributes returned from the expression will be propagated in the set output credentials."]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nRaw string CEL expression. Must return a list of attributes. A maximum of 45 attributes can\nbe selected. Expressions can select different attribute types from attributes:\nattributes.saml_attributes, attributes.iap_attributes."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_credentials` after provisioning.\nWhich output credentials attributes selected by the CEL expression should be propagated in.\nAll attributes will be fully duplicated in each selected output credential.\nPossible values are:\n\n* 'HEADER': Propagate attributes in the headers with \"x-goog-iap-attr-\" prefix.\n* 'JWT': Propagate attributes in the JWT of the form:\n         \"additional_claims\": { \"my_attribute\": [\"value1\", \"value2\"] }\n* 'RCTOKEN': Propagate attributes in the RCToken of the form: \"\n             additional_claims\": { \"my_attribute\": [\"value1\", \"value2\"] } Possible values: [\"HEADER\", \"JWT\", \"RCTOKEN\"]"]
    pub fn output_credentials(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_credentials", self.base))
    }
}

#[derive(Serialize)]
pub struct IapSettings_ApplicationSettings_CsmSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rctoken_aud: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IapSettings_ApplicationSettings_CsmSettings {
    #[doc =
        "Set the field `rctoken_aud`.\nAudience claim set in the generated RCToken. This value is not validated by IAP."]
    pub fn set_rctoken_aud(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rctoken_aud = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IapSettings_ApplicationSettings_CsmSettings {
    type O = terrars::BlockAssignable<IapSettings_ApplicationSettings_CsmSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIapSettings_ApplicationSettings_CsmSettings {}

impl BuildIapSettings_ApplicationSettings_CsmSettings {
    pub fn build(self) -> IapSettings_ApplicationSettings_CsmSettings {
        IapSettings_ApplicationSettings_CsmSettings { rctoken_aud: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct IapSettings_ApplicationSettings_CsmSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IapSettings_ApplicationSettings_CsmSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IapSettings_ApplicationSettings_CsmSettingsRef {
        IapSettings_ApplicationSettings_CsmSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IapSettings_ApplicationSettings_CsmSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `rctoken_aud` after provisioning.\nAudience claim set in the generated RCToken. This value is not validated by IAP."]
    pub fn rctoken_aud(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rctoken_aud", self.base))
    }
}

#[derive(Serialize, Default)]
struct IapSettings_ApplicationSettingsDynamic {
    access_denied_page_settings: std
    ::option
    ::Option<terrars::DynamicBlock<IapSettings_ApplicationSettings_AccessDeniedPageSettings>>,
    attribute_propagation_settings: std
    ::option
    ::Option<terrars::DynamicBlock<IapSettings_ApplicationSettings_AttributePropagationSettings>>,
    csm_settings: std::option::Option<terrars::DynamicBlock<IapSettings_ApplicationSettings_CsmSettings>>,
}

#[derive(Serialize)]
pub struct IapSettings_ApplicationSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_denied_page_settings: std
    ::option
    ::Option<std::vec::Vec<IapSettings_ApplicationSettings_AccessDeniedPageSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_propagation_settings: std
    ::option
    ::Option<std::vec::Vec<IapSettings_ApplicationSettings_AttributePropagationSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csm_settings: std::option::Option<std::vec::Vec<IapSettings_ApplicationSettings_CsmSettings>>,
    dynamic: IapSettings_ApplicationSettingsDynamic,
}

impl IapSettings_ApplicationSettings {
    #[doc =
        "Set the field `cookie_domain`.\nThe Domain value to set for cookies generated by IAP. This value is not validated by the API,\nbut will be ignored at runtime if invalid."]
    pub fn set_cookie_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cookie_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_denied_page_settings`.\n"]
    pub fn set_access_denied_page_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IapSettings_ApplicationSettings_AccessDeniedPageSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_denied_page_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_denied_page_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `attribute_propagation_settings`.\n"]
    pub fn set_attribute_propagation_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IapSettings_ApplicationSettings_AttributePropagationSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.attribute_propagation_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.attribute_propagation_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `csm_settings`.\n"]
    pub fn set_csm_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IapSettings_ApplicationSettings_CsmSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.csm_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.csm_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IapSettings_ApplicationSettings {
    type O = terrars::BlockAssignable<IapSettings_ApplicationSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIapSettings_ApplicationSettings {}

impl BuildIapSettings_ApplicationSettings {
    pub fn build(self) -> IapSettings_ApplicationSettings {
        IapSettings_ApplicationSettings {
            cookie_domain: core::default::Default::default(),
            access_denied_page_settings: core::default::Default::default(),
            attribute_propagation_settings: core::default::Default::default(),
            csm_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IapSettings_ApplicationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IapSettings_ApplicationSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IapSettings_ApplicationSettingsRef {
        IapSettings_ApplicationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IapSettings_ApplicationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cookie_domain` after provisioning.\nThe Domain value to set for cookies generated by IAP. This value is not validated by the API,\nbut will be ignored at runtime if invalid."]
    pub fn cookie_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cookie_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `access_denied_page_settings` after provisioning.\n"]
    pub fn access_denied_page_settings(
        &self,
    ) -> terrars::ListRef<IapSettings_ApplicationSettings_AccessDeniedPageSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_denied_page_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `attribute_propagation_settings` after provisioning.\n"]
    pub fn attribute_propagation_settings(
        &self,
    ) -> terrars::ListRef<IapSettings_ApplicationSettings_AttributePropagationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attribute_propagation_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `csm_settings` after provisioning.\n"]
    pub fn csm_settings(&self) -> terrars::ListRef<IapSettings_ApplicationSettings_CsmSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csm_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct IapSettings_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IapSettings_Timeouts {
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

impl terrars::ToListMappable for IapSettings_Timeouts {
    type O = terrars::BlockAssignable<IapSettings_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIapSettings_Timeouts {}

impl BuildIapSettings_Timeouts {
    pub fn build(self) -> IapSettings_Timeouts {
        IapSettings_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IapSettings_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IapSettings_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IapSettings_TimeoutsRef {
        IapSettings_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IapSettings_TimeoutsRef {
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
