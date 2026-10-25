#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CesDeploymentData_ {
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
    app: terrars::PrimField<std::string::String>,
    app_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel_profile: std::option::Option<std::vec::Vec<CesDeployment_ChannelProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instagram_credentials: std::option::Option<std::vec::Vec<CesDeployment_InstagramCredentials>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CesDeployment_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    whatsapp_credentials: std::option::Option<std::vec::Vec<CesDeployment_WhatsappCredentials>>,
    dynamic: CesDeploymentDynamic,
}

struct CesDeployment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CesDeploymentData_>,
}

#[derive(Clone)]
pub struct CesDeployment(std::rc::Rc<CesDeployment_>);

impl CesDeployment {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `channel_profile`.\n"]
    pub fn set_channel_profile(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesDeployment_ChannelProfile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().channel_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.channel_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instagram_credentials`.\n"]
    pub fn set_instagram_credentials(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesDeployment_InstagramCredentials>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instagram_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instagram_credentials = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CesDeployment_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `whatsapp_credentials`.\n"]
    pub fn set_whatsapp_credentials(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesDeployment_WhatsappCredentials>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().whatsapp_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.whatsapp_credentials = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_version` after provisioning.\nThe resource name of the app version to deploy.\nFormat:\nprojects/{project}/locations/{location}/apps/{app}/versions/{version}"]
    pub fn app_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when this deployment was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the deployment."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the deployment.\nFormat:\nprojects/{project}/locations/{location}/apps/{app}/deployments/{deployment}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when this deployment was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `channel_profile` after provisioning.\n"]
    pub fn channel_profile(&self) -> terrars::ListRef<CesDeployment_ChannelProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.channel_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instagram_credentials` after provisioning.\n"]
    pub fn instagram_credentials(&self) -> terrars::ListRef<CesDeployment_InstagramCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instagram_credentials", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesDeployment_TimeoutsRef {
        CesDeployment_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `whatsapp_credentials` after provisioning.\n"]
    pub fn whatsapp_credentials(&self) -> terrars::ListRef<CesDeployment_WhatsappCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.whatsapp_credentials", self.extract_ref()))
    }
}

impl terrars::Referable for CesDeployment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CesDeployment { }

impl terrars::ToListMappable for CesDeployment {
    type O = terrars::ListRef<CesDeploymentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CesDeployment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_ces_deployment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCesDeployment {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub app: terrars::PrimField<std::string::String>,
    #[doc =
        "The resource name of the app version to deploy.\nFormat:\nprojects/{project}/locations/{location}/apps/{app}/versions/{version}"]
    pub app_version: terrars::PrimField<std::string::String>,
    #[doc = "Display name of the deployment."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildCesDeployment {
    pub fn build(self, stack: &mut terrars::Stack) -> CesDeployment {
        let out = CesDeployment(std::rc::Rc::new(CesDeployment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CesDeploymentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app: self.app,
                app_version: self.app_version,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                channel_profile: core::default::Default::default(),
                instagram_credentials: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                whatsapp_credentials: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CesDeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesDeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CesDeploymentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_version` after provisioning.\nThe resource name of the app version to deploy.\nFormat:\nprojects/{project}/locations/{location}/apps/{app}/versions/{version}"]
    pub fn app_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when this deployment was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the deployment."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the deployment.\nFormat:\nprojects/{project}/locations/{location}/apps/{app}/deployments/{deployment}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when this deployment was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `channel_profile` after provisioning.\n"]
    pub fn channel_profile(&self) -> terrars::ListRef<CesDeployment_ChannelProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.channel_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instagram_credentials` after provisioning.\n"]
    pub fn instagram_credentials(&self) -> terrars::ListRef<CesDeployment_InstagramCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instagram_credentials", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesDeployment_TimeoutsRef {
        CesDeployment_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `whatsapp_credentials` after provisioning.\n"]
    pub fn whatsapp_credentials(&self) -> terrars::ListRef<CesDeployment_WhatsappCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.whatsapp_credentials", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CesDeploymentDynamic {
    channel_profile: std::option::Option<terrars::DynamicBlock<CesDeployment_ChannelProfile>>,
    instagram_credentials: std::option::Option<terrars::DynamicBlock<CesDeployment_InstagramCredentials>>,
    whatsapp_credentials: std::option::Option<terrars::DynamicBlock<CesDeployment_WhatsappCredentials>>,
}

#[derive(Serialize)]
pub struct CesDeployment_ChannelProfile_PersonaProperty {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persona: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesDeployment_ChannelProfile_PersonaProperty {
    #[doc = "Set the field `persona`.\nThe persona of the channel.\nPossible values:\nUNKNOWN\nCONCISE\nCHATTY"]
    pub fn set_persona(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.persona = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesDeployment_ChannelProfile_PersonaProperty {
    type O = terrars::BlockAssignable<CesDeployment_ChannelProfile_PersonaProperty>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesDeployment_ChannelProfile_PersonaProperty {}

impl BuildCesDeployment_ChannelProfile_PersonaProperty {
    pub fn build(self) -> CesDeployment_ChannelProfile_PersonaProperty {
        CesDeployment_ChannelProfile_PersonaProperty { persona: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesDeployment_ChannelProfile_PersonaPropertyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesDeployment_ChannelProfile_PersonaPropertyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesDeployment_ChannelProfile_PersonaPropertyRef {
        CesDeployment_ChannelProfile_PersonaPropertyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesDeployment_ChannelProfile_PersonaPropertyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `persona` after provisioning.\nThe persona of the channel.\nPossible values:\nUNKNOWN\nCONCISE\nCHATTY"]
    pub fn persona(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.persona", self.base))
    }
}

#[derive(Serialize)]
pub struct CesDeployment_ChannelProfile_WebWidgetConfig_SecuritySettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_origins: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_origin_check: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_public_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_recaptcha: std::option::Option<terrars::PrimField<bool>>,
}

impl CesDeployment_ChannelProfile_WebWidgetConfig_SecuritySettings {
    #[doc =
        "Set the field `allowed_origins`.\nThe origins that are allowed to host the web widget. An origin is defined by RFC 6454. If empty, all origins are allowed. A maximum of 100 origins is allowed. Example: \"https://example.com\""]
    pub fn set_allowed_origins(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_origins = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_origin_check`.\nIndicates whether origin check for the web widget is enabled. If true, the web widget will check the origin of the website that loads the web widget and only allow it to be loaded in the same origin or any of the allowed origins."]
    pub fn set_enable_origin_check(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_origin_check = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_public_access`.\nIndicates whether public access to the web widget is enabled. If true, the web widget will be publicly accessible. If false, the web widget must be integrated with your own authentication and authorization system to return valid credentials for accessing the CES agent."]
    pub fn set_enable_public_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_public_access = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_recaptcha`.\nIndicates whether reCAPTCHA verification for the web widget is enabled."]
    pub fn set_enable_recaptcha(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_recaptcha = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesDeployment_ChannelProfile_WebWidgetConfig_SecuritySettings {
    type O = terrars::BlockAssignable<CesDeployment_ChannelProfile_WebWidgetConfig_SecuritySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesDeployment_ChannelProfile_WebWidgetConfig_SecuritySettings {}

impl BuildCesDeployment_ChannelProfile_WebWidgetConfig_SecuritySettings {
    pub fn build(self) -> CesDeployment_ChannelProfile_WebWidgetConfig_SecuritySettings {
        CesDeployment_ChannelProfile_WebWidgetConfig_SecuritySettings {
            allowed_origins: core::default::Default::default(),
            enable_origin_check: core::default::Default::default(),
            enable_public_access: core::default::Default::default(),
            enable_recaptcha: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesDeployment_ChannelProfile_WebWidgetConfig_SecuritySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesDeployment_ChannelProfile_WebWidgetConfig_SecuritySettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesDeployment_ChannelProfile_WebWidgetConfig_SecuritySettingsRef {
        CesDeployment_ChannelProfile_WebWidgetConfig_SecuritySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesDeployment_ChannelProfile_WebWidgetConfig_SecuritySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_origins` after provisioning.\nThe origins that are allowed to host the web widget. An origin is defined by RFC 6454. If empty, all origins are allowed. A maximum of 100 origins is allowed. Example: \"https://example.com\""]
    pub fn allowed_origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_origins", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_origin_check` after provisioning.\nIndicates whether origin check for the web widget is enabled. If true, the web widget will check the origin of the website that loads the web widget and only allow it to be loaded in the same origin or any of the allowed origins."]
    pub fn enable_origin_check(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_origin_check", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_public_access` after provisioning.\nIndicates whether public access to the web widget is enabled. If true, the web widget will be publicly accessible. If false, the web widget must be integrated with your own authentication and authorization system to return valid credentials for accessing the CES agent."]
    pub fn enable_public_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_public_access", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_recaptcha` after provisioning.\nIndicates whether reCAPTCHA verification for the web widget is enabled."]
    pub fn enable_recaptcha(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_recaptcha", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesDeployment_ChannelProfile_WebWidgetConfigDynamic {
    security_settings: std
    ::option
    ::Option<terrars::DynamicBlock<CesDeployment_ChannelProfile_WebWidgetConfig_SecuritySettings>>,
}

#[derive(Serialize)]
pub struct CesDeployment_ChannelProfile_WebWidgetConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    modality: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    theme: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_widget_title: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_settings: std::option::Option<std::vec::Vec<CesDeployment_ChannelProfile_WebWidgetConfig_SecuritySettings>>,
    dynamic: CesDeployment_ChannelProfile_WebWidgetConfigDynamic,
}

impl CesDeployment_ChannelProfile_WebWidgetConfig {
    #[doc =
        "Set the field `modality`.\nThe modality of the web widget.\nPossible values:\nMODALITY_UNSPECIFIED\nCHAT_AND_VOICE\nVOICE_ONLY\nCHAT_ONLY\nCHAT_VOICE_AND_VIDEO"]
    pub fn set_modality(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.modality = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `theme`.\nThe theme of the web widget.\nPossible values:\nTHEME_UNSPECIFIED\nLIGHT\nDARK"]
    pub fn set_theme(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.theme = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `web_widget_title`.\nThe title of the web widget."]
    pub fn set_web_widget_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.web_widget_title = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_settings`.\n"]
    pub fn set_security_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CesDeployment_ChannelProfile_WebWidgetConfig_SecuritySettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.security_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.security_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesDeployment_ChannelProfile_WebWidgetConfig {
    type O = terrars::BlockAssignable<CesDeployment_ChannelProfile_WebWidgetConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesDeployment_ChannelProfile_WebWidgetConfig {}

impl BuildCesDeployment_ChannelProfile_WebWidgetConfig {
    pub fn build(self) -> CesDeployment_ChannelProfile_WebWidgetConfig {
        CesDeployment_ChannelProfile_WebWidgetConfig {
            modality: core::default::Default::default(),
            theme: core::default::Default::default(),
            web_widget_title: core::default::Default::default(),
            security_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesDeployment_ChannelProfile_WebWidgetConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesDeployment_ChannelProfile_WebWidgetConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesDeployment_ChannelProfile_WebWidgetConfigRef {
        CesDeployment_ChannelProfile_WebWidgetConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesDeployment_ChannelProfile_WebWidgetConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `modality` after provisioning.\nThe modality of the web widget.\nPossible values:\nMODALITY_UNSPECIFIED\nCHAT_AND_VOICE\nVOICE_ONLY\nCHAT_ONLY\nCHAT_VOICE_AND_VIDEO"]
    pub fn modality(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modality", self.base))
    }

    #[doc =
        "Get a reference to the value of field `theme` after provisioning.\nThe theme of the web widget.\nPossible values:\nTHEME_UNSPECIFIED\nLIGHT\nDARK"]
    pub fn theme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.theme", self.base))
    }

    #[doc =
        "Get a reference to the value of field `web_widget_title` after provisioning.\nThe title of the web widget."]
    pub fn web_widget_title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_widget_title", self.base))
    }

    #[doc = "Get a reference to the value of field `security_settings` after provisioning.\n"]
    pub fn security_settings(
        &self,
    ) -> terrars::ListRef<CesDeployment_ChannelProfile_WebWidgetConfig_SecuritySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct CesDeployment_ChannelProfile_WhatsappConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_number: std::option::Option<terrars::PrimField<std::string::String>>,
    phone_number_id: terrars::PrimField<std::string::String>,
    waba_id: terrars::PrimField<std::string::String>,
}

impl CesDeployment_ChannelProfile_WhatsappConfig {
    #[doc = "Set the field `phone_number`.\nOptional. The phone number in E.164 format."]
    pub fn set_phone_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.phone_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesDeployment_ChannelProfile_WhatsappConfig {
    type O = terrars::BlockAssignable<CesDeployment_ChannelProfile_WhatsappConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesDeployment_ChannelProfile_WhatsappConfig {
    #[doc = "Required. The Meta phone number ID."]
    pub phone_number_id: terrars::PrimField<std::string::String>,
    #[doc = "Required. The WhatsApp Business Account ID."]
    pub waba_id: terrars::PrimField<std::string::String>,
}

impl BuildCesDeployment_ChannelProfile_WhatsappConfig {
    pub fn build(self) -> CesDeployment_ChannelProfile_WhatsappConfig {
        CesDeployment_ChannelProfile_WhatsappConfig {
            phone_number: core::default::Default::default(),
            phone_number_id: self.phone_number_id,
            waba_id: self.waba_id,
        }
    }
}

#[allow(dead_code)]
pub struct CesDeployment_ChannelProfile_WhatsappConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesDeployment_ChannelProfile_WhatsappConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesDeployment_ChannelProfile_WhatsappConfigRef {
        CesDeployment_ChannelProfile_WhatsappConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesDeployment_ChannelProfile_WhatsappConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOutput only. The description of the Meta business page or profile."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOutput only. The fetched Meta business page name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `phone_number` after provisioning.\nOptional. The phone number in E.164 format."]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }

    #[doc =
        "Get a reference to the value of field `phone_number_id` after provisioning.\nRequired. The Meta phone number ID."]
    pub fn phone_number_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `thumbnail_url` after provisioning.\nOutput only. The fetched Meta business profile thumbnail URL."]
    pub fn thumbnail_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thumbnail_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `waba_id` after provisioning.\nRequired. The WhatsApp Business Account ID."]
    pub fn waba_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.waba_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesDeployment_ChannelProfileDynamic {
    persona_property: std::option::Option<terrars::DynamicBlock<CesDeployment_ChannelProfile_PersonaProperty>>,
    web_widget_config: std::option::Option<terrars::DynamicBlock<CesDeployment_ChannelProfile_WebWidgetConfig>>,
    whatsapp_config: std::option::Option<terrars::DynamicBlock<CesDeployment_ChannelProfile_WhatsappConfig>>,
}

#[derive(Serialize)]
pub struct CesDeployment_ChannelProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_barge_in_control: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_dtmf: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persona_property: std::option::Option<std::vec::Vec<CesDeployment_ChannelProfile_PersonaProperty>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_widget_config: std::option::Option<std::vec::Vec<CesDeployment_ChannelProfile_WebWidgetConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    whatsapp_config: std::option::Option<std::vec::Vec<CesDeployment_ChannelProfile_WhatsappConfig>>,
    dynamic: CesDeployment_ChannelProfileDynamic,
}

impl CesDeployment_ChannelProfile {
    #[doc =
        "Set the field `channel_type`.\nThe type of the channel profile.\nPossible values:\nUNKNOWN\nWEB_UI\nAPI\nTWILIO\nGOOGLE_TELEPHONY_PLATFORM\nCONTACT_CENTER_AS_A_SERVICE\nFIVE9\nCONTACT_CENTER_INTEGRATION\nWHATSAPP\nINSTAGRAM"]
    pub fn set_channel_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.channel_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_barge_in_control`.\nWhether to disable user barge-in control in the conversation.\n- **true**: User interruptions are disabled while the agent is speaking.\n- **false**: The agent retains automatic control over when the user can\ninterrupt."]
    pub fn set_disable_barge_in_control(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_barge_in_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_dtmf`.\nWhether to disable DTMF (dual-tone multi-frequency)."]
    pub fn set_disable_dtmf(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_dtmf = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `profile_id`.\nThe unique identifier of the channel profile."]
    pub fn set_profile_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.profile_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `persona_property`.\n"]
    pub fn set_persona_property(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesDeployment_ChannelProfile_PersonaProperty>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.persona_property = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.persona_property = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `web_widget_config`.\n"]
    pub fn set_web_widget_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesDeployment_ChannelProfile_WebWidgetConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.web_widget_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.web_widget_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `whatsapp_config`.\n"]
    pub fn set_whatsapp_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesDeployment_ChannelProfile_WhatsappConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.whatsapp_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.whatsapp_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesDeployment_ChannelProfile {
    type O = terrars::BlockAssignable<CesDeployment_ChannelProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesDeployment_ChannelProfile {}

impl BuildCesDeployment_ChannelProfile {
    pub fn build(self) -> CesDeployment_ChannelProfile {
        CesDeployment_ChannelProfile {
            channel_type: core::default::Default::default(),
            disable_barge_in_control: core::default::Default::default(),
            disable_dtmf: core::default::Default::default(),
            profile_id: core::default::Default::default(),
            persona_property: core::default::Default::default(),
            web_widget_config: core::default::Default::default(),
            whatsapp_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesDeployment_ChannelProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesDeployment_ChannelProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesDeployment_ChannelProfileRef {
        CesDeployment_ChannelProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesDeployment_ChannelProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `channel_type` after provisioning.\nThe type of the channel profile.\nPossible values:\nUNKNOWN\nWEB_UI\nAPI\nTWILIO\nGOOGLE_TELEPHONY_PLATFORM\nCONTACT_CENTER_AS_A_SERVICE\nFIVE9\nCONTACT_CENTER_INTEGRATION\nWHATSAPP\nINSTAGRAM"]
    pub fn channel_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_barge_in_control` after provisioning.\nWhether to disable user barge-in control in the conversation.\n- **true**: User interruptions are disabled while the agent is speaking.\n- **false**: The agent retains automatic control over when the user can\ninterrupt."]
    pub fn disable_barge_in_control(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_barge_in_control", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_dtmf` after provisioning.\nWhether to disable DTMF (dual-tone multi-frequency)."]
    pub fn disable_dtmf(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_dtmf", self.base))
    }

    #[doc =
        "Get a reference to the value of field `profile_id` after provisioning.\nThe unique identifier of the channel profile."]
    pub fn profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_id", self.base))
    }

    #[doc = "Get a reference to the value of field `persona_property` after provisioning.\n"]
    pub fn persona_property(&self) -> terrars::ListRef<CesDeployment_ChannelProfile_PersonaPropertyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persona_property", self.base))
    }

    #[doc = "Get a reference to the value of field `web_widget_config` after provisioning.\n"]
    pub fn web_widget_config(&self) -> terrars::ListRef<CesDeployment_ChannelProfile_WebWidgetConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_widget_config", self.base))
    }

    #[doc = "Get a reference to the value of field `whatsapp_config` after provisioning.\n"]
    pub fn whatsapp_config(&self) -> terrars::ListRef<CesDeployment_ChannelProfile_WhatsappConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.whatsapp_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesDeployment_InstagramCredentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_code_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_code_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_profile_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesDeployment_InstagramCredentials {
    #[doc = "Set the field `auth_code`.\nThe Meta auth code provided by the embedded signup flow."]
    pub fn set_auth_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_code_wo`.\nThe Meta auth code provided by the embedded signup flow."]
    pub fn set_auth_code_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_code_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `auth_code_wo_version`.\nTriggers update of 'auth_code_wo' write-only. Increment this value when an update to 'auth_code_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_auth_code_wo_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.auth_code_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conversation_profile_id`.\nThe Conversation Profile ID to use for the deployment."]
    pub fn set_conversation_profile_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.conversation_profile_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesDeployment_InstagramCredentials {
    type O = terrars::BlockAssignable<CesDeployment_InstagramCredentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesDeployment_InstagramCredentials {}

impl BuildCesDeployment_InstagramCredentials {
    pub fn build(self) -> CesDeployment_InstagramCredentials {
        CesDeployment_InstagramCredentials {
            auth_code: core::default::Default::default(),
            auth_code_wo: core::default::Default::default(),
            auth_code_wo_version: core::default::Default::default(),
            conversation_profile_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesDeployment_InstagramCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesDeployment_InstagramCredentialsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesDeployment_InstagramCredentialsRef {
        CesDeployment_InstagramCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesDeployment_InstagramCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_code` after provisioning.\nThe Meta auth code provided by the embedded signup flow."]
    pub fn auth_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `auth_code_wo` after provisioning.\nThe Meta auth code provided by the embedded signup flow."]
    pub fn auth_code_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_code_wo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `auth_code_wo_version` after provisioning.\nTriggers update of 'auth_code_wo' write-only. Increment this value when an update to 'auth_code_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn auth_code_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_code_wo_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `conversation_profile_id` after provisioning.\nThe Conversation Profile ID to use for the deployment."]
    pub fn conversation_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conversation_profile_id", self.base))
    }
}

#[derive(Serialize)]
pub struct CesDeployment_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesDeployment_Timeouts {
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

impl terrars::ToListMappable for CesDeployment_Timeouts {
    type O = terrars::BlockAssignable<CesDeployment_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesDeployment_Timeouts {}

impl BuildCesDeployment_Timeouts {
    pub fn build(self) -> CesDeployment_Timeouts {
        CesDeployment_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesDeployment_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesDeployment_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesDeployment_TimeoutsRef {
        CesDeployment_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesDeployment_TimeoutsRef {
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
pub struct CesDeployment_WhatsappCredentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_code_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_code_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
    business_account_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_profile_id: std::option::Option<terrars::PrimField<std::string::String>>,
    phone_number: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pin: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pin_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pin_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
    waba_id: terrars::PrimField<std::string::String>,
}

impl CesDeployment_WhatsappCredentials {
    #[doc = "Set the field `auth_code`.\nThe Meta auth code provided by the embedded signup flow."]
    pub fn set_auth_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_code_wo`.\nThe Meta auth code provided by the embedded signup flow."]
    pub fn set_auth_code_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_code_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `auth_code_wo_version`.\nTriggers update of 'auth_code_wo' write-only. Increment this value when an update to 'auth_code_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_auth_code_wo_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.auth_code_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conversation_profile_id`.\nThe Conversation Profile ID to use for the deployment."]
    pub fn set_conversation_profile_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.conversation_profile_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pin`.\nThe 6-digit PIN created by the user for two-step verification."]
    pub fn set_pin(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pin_wo`.\nThe 6-digit PIN created by the user for two-step verification."]
    pub fn set_pin_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pin_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pin_wo_version`.\nTriggers update of 'pin_wo' write-only. Increment this value when an update to 'pin_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_pin_wo_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pin_wo_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesDeployment_WhatsappCredentials {
    type O = terrars::BlockAssignable<CesDeployment_WhatsappCredentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesDeployment_WhatsappCredentials {
    #[doc = "The Business Account ID to use for the phone number."]
    pub business_account_id: terrars::PrimField<std::string::String>,
    #[doc = "The phone number to register with WhatsApp."]
    pub phone_number: terrars::PrimField<std::string::String>,
    #[doc = "The WhatsApp Business Account ID."]
    pub waba_id: terrars::PrimField<std::string::String>,
}

impl BuildCesDeployment_WhatsappCredentials {
    pub fn build(self) -> CesDeployment_WhatsappCredentials {
        CesDeployment_WhatsappCredentials {
            auth_code: core::default::Default::default(),
            auth_code_wo: core::default::Default::default(),
            auth_code_wo_version: core::default::Default::default(),
            business_account_id: self.business_account_id,
            conversation_profile_id: core::default::Default::default(),
            phone_number: self.phone_number,
            pin: core::default::Default::default(),
            pin_wo: core::default::Default::default(),
            pin_wo_version: core::default::Default::default(),
            waba_id: self.waba_id,
        }
    }
}

#[allow(dead_code)]
pub struct CesDeployment_WhatsappCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesDeployment_WhatsappCredentialsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesDeployment_WhatsappCredentialsRef {
        CesDeployment_WhatsappCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesDeployment_WhatsappCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_code` after provisioning.\nThe Meta auth code provided by the embedded signup flow."]
    pub fn auth_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `auth_code_wo` after provisioning.\nThe Meta auth code provided by the embedded signup flow."]
    pub fn auth_code_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_code_wo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `auth_code_wo_version` after provisioning.\nTriggers update of 'auth_code_wo' write-only. Increment this value when an update to 'auth_code_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn auth_code_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_code_wo_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `business_account_id` after provisioning.\nThe Business Account ID to use for the phone number."]
    pub fn business_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_account_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `conversation_profile_id` after provisioning.\nThe Conversation Profile ID to use for the deployment."]
    pub fn conversation_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conversation_profile_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `phone_number` after provisioning.\nThe phone number to register with WhatsApp."]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pin` after provisioning.\nThe 6-digit PIN created by the user for two-step verification."]
    pub fn pin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pin", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pin_wo` after provisioning.\nThe 6-digit PIN created by the user for two-step verification."]
    pub fn pin_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pin_wo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pin_wo_version` after provisioning.\nTriggers update of 'pin_wo' write-only. Increment this value when an update to 'pin_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn pin_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pin_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `waba_id` after provisioning.\nThe WhatsApp Business Account ID."]
    pub fn waba_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.waba_id", self.base))
    }
}
