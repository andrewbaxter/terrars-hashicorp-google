use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct LookerInstanceData {
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
    consumer_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform_edition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ip_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_ip_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reserved_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_settings: std::option::Option<std::vec::Vec<LookerInstanceAdminSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny_maintenance_period: std::option::Option<std::vec::Vec<LookerInstanceDenyMaintenancePeriodEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<std::vec::Vec<LookerInstanceEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std::option::Option<std::vec::Vec<LookerInstanceMaintenanceWindowEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_config: std::option::Option<std::vec::Vec<LookerInstanceOauthConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LookerInstanceTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_metadata: std::option::Option<std::vec::Vec<LookerInstanceUserMetadataEl>>,
    dynamic: LookerInstanceDynamic,
}

struct LookerInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LookerInstanceData>,
}

#[derive(Clone)]
pub struct LookerInstance(std::rc::Rc<LookerInstance_>);

impl LookerInstance {
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
        "Set the field `consumer_network`.\nNetwork name in the consumer project in the format of: projects/{project}/global/networks/{network}\nNote that the consumer network may be in a different GCP project than the consumer\nproject that is hosting the Looker Instance."]
    pub fn set_consumer_network(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().consumer_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `platform_edition`.\nPlatform editions for a Looker instance. Each edition maps to a set of instance features, like its size. Must be one of these values:\n- LOOKER_CORE_TRIAL: trial instance\n- LOOKER_CORE_STANDARD: pay as you go standard instance\n- LOOKER_CORE_STANDARD_ANNUAL: subscription standard instance\n- LOOKER_CORE_ENTERPRISE_ANNUAL: subscription enterprise instance\n- LOOKER_CORE_EMBED_ANNUAL: subscription embed instance Default value: \"LOOKER_CORE_TRIAL\" Possible values: [\"LOOKER_CORE_TRIAL\", \"LOOKER_CORE_STANDARD\", \"LOOKER_CORE_STANDARD_ANNUAL\", \"LOOKER_CORE_ENTERPRISE_ANNUAL\", \"LOOKER_CORE_EMBED_ANNUAL\"]"]
    pub fn set_platform_edition(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().platform_edition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_ip_enabled`.\nWhether private IP is enabled on the Looker instance."]
    pub fn set_private_ip_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().private_ip_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_ip_enabled`.\nWhether public IP is enabled on the Looker instance."]
    pub fn set_public_ip_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().public_ip_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe name of the Looker region of the instance."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `reserved_range`.\nName of a reserved IP address range within the consumer network, to be used for\nprivate service access connection. User may or may not specify this in a request."]
    pub fn set_reserved_range(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().reserved_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `admin_settings`.\n"]
    pub fn set_admin_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstanceAdminSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().admin_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.admin_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deny_maintenance_period`.\n"]
    pub fn set_deny_maintenance_period(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstanceDenyMaintenancePeriodEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deny_maintenance_period = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deny_maintenance_period = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstanceEncryptionConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstanceMaintenanceWindowEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_window = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth_config`.\n"]
    pub fn set_oauth_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstanceOauthConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oauth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oauth_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LookerInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_metadata`.\n"]
    pub fn set_user_metadata(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstanceUserMetadataEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_metadata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `consumer_network` after provisioning.\nNetwork name in the consumer project in the format of: projects/{project}/global/networks/{network}\nNote that the consumer network may be in a different GCP project than the consumer\nproject that is hosting the Looker Instance."]
    pub fn consumer_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the instance was created in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egress_public_ip` after provisioning.\nPublic Egress IP (IPv4)."]
    pub fn egress_public_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.egress_public_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ingress_private_ip` after provisioning.\nPrivate Ingress IP (IPv4)."]
    pub fn ingress_private_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_private_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress_public_ip` after provisioning.\nPublic Ingress IP (IPv4)."]
    pub fn ingress_public_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_public_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `looker_uri` after provisioning.\nLooker instance URI which can be used to access the Looker Instance UI."]
    pub fn looker_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.looker_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `looker_version` after provisioning.\nThe Looker version that the instance is using."]
    pub fn looker_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.looker_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe ID of the instance or a fully qualified identifier for the instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `platform_edition` after provisioning.\nPlatform editions for a Looker instance. Each edition maps to a set of instance features, like its size. Must be one of these values:\n- LOOKER_CORE_TRIAL: trial instance\n- LOOKER_CORE_STANDARD: pay as you go standard instance\n- LOOKER_CORE_STANDARD_ANNUAL: subscription standard instance\n- LOOKER_CORE_ENTERPRISE_ANNUAL: subscription enterprise instance\n- LOOKER_CORE_EMBED_ANNUAL: subscription embed instance Default value: \"LOOKER_CORE_TRIAL\" Possible values: [\"LOOKER_CORE_TRIAL\", \"LOOKER_CORE_STANDARD\", \"LOOKER_CORE_STANDARD_ANNUAL\", \"LOOKER_CORE_ENTERPRISE_ANNUAL\", \"LOOKER_CORE_EMBED_ANNUAL\"]"]
    pub fn platform_edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_edition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_ip_enabled` after provisioning.\nWhether private IP is enabled on the Looker instance."]
    pub fn private_ip_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_ip_enabled` after provisioning.\nWhether public IP is enabled on the Looker instance."]
    pub fn public_ip_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe name of the Looker region of the instance."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reserved_range` after provisioning.\nName of a reserved IP address range within the consumer network, to be used for\nprivate service access connection. User may or may not specify this in a request."]
    pub fn reserved_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reserved_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the instance was updated in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_settings` after provisioning.\n"]
    pub fn admin_settings(&self) -> terrars::ListRef<LookerInstanceAdminSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deny_maintenance_period` after provisioning.\n"]
    pub fn deny_maintenance_period(&self) -> terrars::ListRef<LookerInstanceDenyMaintenancePeriodElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deny_maintenance_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<LookerInstanceEncryptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<LookerInstanceMaintenanceWindowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(&self) -> terrars::ListRef<LookerInstanceOauthConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LookerInstanceTimeoutsElRef {
        LookerInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_metadata` after provisioning.\n"]
    pub fn user_metadata(&self) -> terrars::ListRef<LookerInstanceUserMetadataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_metadata", self.extract_ref()))
    }
}

impl terrars::Referable for LookerInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LookerInstance { }

impl terrars::ToListMappable for LookerInstance {
    type O = terrars::ListRef<LookerInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LookerInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_looker_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLookerInstance {
    pub tf_id: std::string::String,
    #[doc = "The ID of the instance or a fully qualified identifier for the instance."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLookerInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> LookerInstance {
        let out = LookerInstance(std::rc::Rc::new(LookerInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LookerInstanceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                consumer_network: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                platform_edition: core::default::Default::default(),
                private_ip_enabled: core::default::Default::default(),
                project: core::default::Default::default(),
                public_ip_enabled: core::default::Default::default(),
                region: core::default::Default::default(),
                reserved_range: core::default::Default::default(),
                admin_settings: core::default::Default::default(),
                deny_maintenance_period: core::default::Default::default(),
                encryption_config: core::default::Default::default(),
                maintenance_window: core::default::Default::default(),
                oauth_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                user_metadata: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LookerInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LookerInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `consumer_network` after provisioning.\nNetwork name in the consumer project in the format of: projects/{project}/global/networks/{network}\nNote that the consumer network may be in a different GCP project than the consumer\nproject that is hosting the Looker Instance."]
    pub fn consumer_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the instance was created in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egress_public_ip` after provisioning.\nPublic Egress IP (IPv4)."]
    pub fn egress_public_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.egress_public_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ingress_private_ip` after provisioning.\nPrivate Ingress IP (IPv4)."]
    pub fn ingress_private_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_private_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress_public_ip` after provisioning.\nPublic Ingress IP (IPv4)."]
    pub fn ingress_public_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_public_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `looker_uri` after provisioning.\nLooker instance URI which can be used to access the Looker Instance UI."]
    pub fn looker_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.looker_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `looker_version` after provisioning.\nThe Looker version that the instance is using."]
    pub fn looker_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.looker_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe ID of the instance or a fully qualified identifier for the instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `platform_edition` after provisioning.\nPlatform editions for a Looker instance. Each edition maps to a set of instance features, like its size. Must be one of these values:\n- LOOKER_CORE_TRIAL: trial instance\n- LOOKER_CORE_STANDARD: pay as you go standard instance\n- LOOKER_CORE_STANDARD_ANNUAL: subscription standard instance\n- LOOKER_CORE_ENTERPRISE_ANNUAL: subscription enterprise instance\n- LOOKER_CORE_EMBED_ANNUAL: subscription embed instance Default value: \"LOOKER_CORE_TRIAL\" Possible values: [\"LOOKER_CORE_TRIAL\", \"LOOKER_CORE_STANDARD\", \"LOOKER_CORE_STANDARD_ANNUAL\", \"LOOKER_CORE_ENTERPRISE_ANNUAL\", \"LOOKER_CORE_EMBED_ANNUAL\"]"]
    pub fn platform_edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_edition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_ip_enabled` after provisioning.\nWhether private IP is enabled on the Looker instance."]
    pub fn private_ip_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_ip_enabled` after provisioning.\nWhether public IP is enabled on the Looker instance."]
    pub fn public_ip_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe name of the Looker region of the instance."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reserved_range` after provisioning.\nName of a reserved IP address range within the consumer network, to be used for\nprivate service access connection. User may or may not specify this in a request."]
    pub fn reserved_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reserved_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the instance was updated in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_settings` after provisioning.\n"]
    pub fn admin_settings(&self) -> terrars::ListRef<LookerInstanceAdminSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deny_maintenance_period` after provisioning.\n"]
    pub fn deny_maintenance_period(&self) -> terrars::ListRef<LookerInstanceDenyMaintenancePeriodElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deny_maintenance_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<LookerInstanceEncryptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<LookerInstanceMaintenanceWindowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(&self) -> terrars::ListRef<LookerInstanceOauthConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LookerInstanceTimeoutsElRef {
        LookerInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_metadata` after provisioning.\n"]
    pub fn user_metadata(&self) -> terrars::ListRef<LookerInstanceUserMetadataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_metadata", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LookerInstanceAdminSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_email_domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl LookerInstanceAdminSettingsEl {
    #[doc =
        "Set the field `allowed_email_domains`.\nEmail domain allowlist for the instance.\n\nDefine the email domains to which your users can deliver Looker (Google Cloud core) content.\nUpdating this list will restart the instance. Updating the allowed email domains from terraform\nmeans the value provided will be considered as the entire list and not an amendment to the\nexisting list of allowed email domains."]
    pub fn set_allowed_email_domains(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_email_domains = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LookerInstanceAdminSettingsEl {
    type O = terrars::BlockAssignable<LookerInstanceAdminSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstanceAdminSettingsEl {}

impl BuildLookerInstanceAdminSettingsEl {
    pub fn build(self) -> LookerInstanceAdminSettingsEl {
        LookerInstanceAdminSettingsEl { allowed_email_domains: core::default::Default::default() }
    }
}

pub struct LookerInstanceAdminSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstanceAdminSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstanceAdminSettingsElRef {
        LookerInstanceAdminSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceAdminSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_email_domains` after provisioning.\nEmail domain allowlist for the instance.\n\nDefine the email domains to which your users can deliver Looker (Google Cloud core) content.\nUpdating this list will restart the instance. Updating the allowed email domains from terraform\nmeans the value provided will be considered as the entire list and not an amendment to the\nexisting list of allowed email domains."]
    pub fn allowed_email_domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_email_domains", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstanceDenyMaintenancePeriodElEndDateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    month: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    year: std::option::Option<terrars::PrimField<f64>>,
}

impl LookerInstanceDenyMaintenancePeriodElEndDateEl {
    #[doc =
        "Set the field `day`.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't significant."]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `month`.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn set_month(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `year`.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn set_year(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.year = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LookerInstanceDenyMaintenancePeriodElEndDateEl {
    type O = terrars::BlockAssignable<LookerInstanceDenyMaintenancePeriodElEndDateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstanceDenyMaintenancePeriodElEndDateEl {}

impl BuildLookerInstanceDenyMaintenancePeriodElEndDateEl {
    pub fn build(self) -> LookerInstanceDenyMaintenancePeriodElEndDateEl {
        LookerInstanceDenyMaintenancePeriodElEndDateEl {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}

pub struct LookerInstanceDenyMaintenancePeriodElEndDateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstanceDenyMaintenancePeriodElEndDateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LookerInstanceDenyMaintenancePeriodElEndDateElRef {
        LookerInstanceDenyMaintenancePeriodElEndDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceDenyMaintenancePeriodElEndDateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't significant."]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc =
        "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstanceDenyMaintenancePeriodElStartDateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    month: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    year: std::option::Option<terrars::PrimField<f64>>,
}

impl LookerInstanceDenyMaintenancePeriodElStartDateEl {
    #[doc =
        "Set the field `day`.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't significant."]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `month`.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn set_month(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `year`.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn set_year(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.year = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LookerInstanceDenyMaintenancePeriodElStartDateEl {
    type O = terrars::BlockAssignable<LookerInstanceDenyMaintenancePeriodElStartDateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstanceDenyMaintenancePeriodElStartDateEl {}

impl BuildLookerInstanceDenyMaintenancePeriodElStartDateEl {
    pub fn build(self) -> LookerInstanceDenyMaintenancePeriodElStartDateEl {
        LookerInstanceDenyMaintenancePeriodElStartDateEl {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}

pub struct LookerInstanceDenyMaintenancePeriodElStartDateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstanceDenyMaintenancePeriodElStartDateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LookerInstanceDenyMaintenancePeriodElStartDateElRef {
        LookerInstanceDenyMaintenancePeriodElStartDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceDenyMaintenancePeriodElStartDateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't significant."]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc =
        "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstanceDenyMaintenancePeriodElTimeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl LookerInstanceDenyMaintenancePeriodElTimeEl {
    #[doc = "Set the field `hours`.\nHours of day in 24 hour format. Should be from 0 to 23."]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\nSeconds of minutes of the time. Must normally be from 0 to 59."]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LookerInstanceDenyMaintenancePeriodElTimeEl {
    type O = terrars::BlockAssignable<LookerInstanceDenyMaintenancePeriodElTimeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstanceDenyMaintenancePeriodElTimeEl {}

impl BuildLookerInstanceDenyMaintenancePeriodElTimeEl {
    pub fn build(self) -> LookerInstanceDenyMaintenancePeriodElTimeEl {
        LookerInstanceDenyMaintenancePeriodElTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

pub struct LookerInstanceDenyMaintenancePeriodElTimeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstanceDenyMaintenancePeriodElTimeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LookerInstanceDenyMaintenancePeriodElTimeElRef {
        LookerInstanceDenyMaintenancePeriodElTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceDenyMaintenancePeriodElTimeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hours` after provisioning.\nHours of day in 24 hour format. Should be from 0 to 23."]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minutes` after provisioning.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSeconds of minutes of the time. Must normally be from 0 to 59."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct LookerInstanceDenyMaintenancePeriodElDynamic {
    end_date: std::option::Option<terrars::DynamicBlock<LookerInstanceDenyMaintenancePeriodElEndDateEl>>,
    start_date: std::option::Option<terrars::DynamicBlock<LookerInstanceDenyMaintenancePeriodElStartDateEl>>,
    time: std::option::Option<terrars::DynamicBlock<LookerInstanceDenyMaintenancePeriodElTimeEl>>,
}

#[derive(Serialize)]
pub struct LookerInstanceDenyMaintenancePeriodEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_date: std::option::Option<std::vec::Vec<LookerInstanceDenyMaintenancePeriodElEndDateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_date: std::option::Option<std::vec::Vec<LookerInstanceDenyMaintenancePeriodElStartDateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time: std::option::Option<std::vec::Vec<LookerInstanceDenyMaintenancePeriodElTimeEl>>,
    dynamic: LookerInstanceDenyMaintenancePeriodElDynamic,
}

impl LookerInstanceDenyMaintenancePeriodEl {
    #[doc = "Set the field `end_date`.\n"]
    pub fn set_end_date(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstanceDenyMaintenancePeriodElEndDateEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.end_date = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.end_date = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `start_date`.\n"]
    pub fn set_start_date(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstanceDenyMaintenancePeriodElStartDateEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start_date = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start_date = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time`.\n"]
    pub fn set_time(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstanceDenyMaintenancePeriodElTimeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LookerInstanceDenyMaintenancePeriodEl {
    type O = terrars::BlockAssignable<LookerInstanceDenyMaintenancePeriodEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstanceDenyMaintenancePeriodEl {}

impl BuildLookerInstanceDenyMaintenancePeriodEl {
    pub fn build(self) -> LookerInstanceDenyMaintenancePeriodEl {
        LookerInstanceDenyMaintenancePeriodEl {
            end_date: core::default::Default::default(),
            start_date: core::default::Default::default(),
            time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LookerInstanceDenyMaintenancePeriodElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstanceDenyMaintenancePeriodElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstanceDenyMaintenancePeriodElRef {
        LookerInstanceDenyMaintenancePeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceDenyMaintenancePeriodElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> terrars::ListRef<LookerInstanceDenyMaintenancePeriodElEndDateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end_date", self.base))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> terrars::ListRef<LookerInstanceDenyMaintenancePeriodElStartDateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_date", self.base))
    }

    #[doc = "Get a reference to the value of field `time` after provisioning.\n"]
    pub fn time(&self) -> terrars::ListRef<LookerInstanceDenyMaintenancePeriodElTimeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstanceEncryptionConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LookerInstanceEncryptionConfigEl {
    #[doc = "Set the field `kms_key_name`.\nName of the customer managed encryption key (CMEK) in KMS."]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LookerInstanceEncryptionConfigEl {
    type O = terrars::BlockAssignable<LookerInstanceEncryptionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstanceEncryptionConfigEl {}

impl BuildLookerInstanceEncryptionConfigEl {
    pub fn build(self) -> LookerInstanceEncryptionConfigEl {
        LookerInstanceEncryptionConfigEl { kms_key_name: core::default::Default::default() }
    }
}

pub struct LookerInstanceEncryptionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstanceEncryptionConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstanceEncryptionConfigElRef {
        LookerInstanceEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceEncryptionConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nName of the customer managed encryption key (CMEK) in KMS."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name_version` after provisioning.\nFull name and version of the CMEK key currently in use to encrypt Looker data."]
    pub fn kms_key_name_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_state` after provisioning.\nStatus of the customer managed encryption key (CMEK) in KMS."]
    pub fn kms_key_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_state", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstanceMaintenanceWindowElStartTimeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl LookerInstanceMaintenanceWindowElStartTimeEl {
    #[doc = "Set the field `hours`.\nHours of day in 24 hour format. Should be from 0 to 23."]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\nSeconds of minutes of the time. Must normally be from 0 to 59."]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LookerInstanceMaintenanceWindowElStartTimeEl {
    type O = terrars::BlockAssignable<LookerInstanceMaintenanceWindowElStartTimeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstanceMaintenanceWindowElStartTimeEl {}

impl BuildLookerInstanceMaintenanceWindowElStartTimeEl {
    pub fn build(self) -> LookerInstanceMaintenanceWindowElStartTimeEl {
        LookerInstanceMaintenanceWindowElStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

pub struct LookerInstanceMaintenanceWindowElStartTimeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstanceMaintenanceWindowElStartTimeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LookerInstanceMaintenanceWindowElStartTimeElRef {
        LookerInstanceMaintenanceWindowElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceMaintenanceWindowElStartTimeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hours` after provisioning.\nHours of day in 24 hour format. Should be from 0 to 23."]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minutes` after provisioning.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSeconds of minutes of the time. Must normally be from 0 to 59."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct LookerInstanceMaintenanceWindowElDynamic {
    start_time: std::option::Option<terrars::DynamicBlock<LookerInstanceMaintenanceWindowElStartTimeEl>>,
}

#[derive(Serialize)]
pub struct LookerInstanceMaintenanceWindowEl {
    day_of_week: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<std::vec::Vec<LookerInstanceMaintenanceWindowElStartTimeEl>>,
    dynamic: LookerInstanceMaintenanceWindowElDynamic,
}

impl LookerInstanceMaintenanceWindowEl {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstanceMaintenanceWindowElStartTimeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LookerInstanceMaintenanceWindowEl {
    type O = terrars::BlockAssignable<LookerInstanceMaintenanceWindowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstanceMaintenanceWindowEl {
    #[doc =
        "Required. Day of the week for this MaintenanceWindow (in UTC).\n\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day_of_week: terrars::PrimField<std::string::String>,
}

impl BuildLookerInstanceMaintenanceWindowEl {
    pub fn build(self) -> LookerInstanceMaintenanceWindowEl {
        LookerInstanceMaintenanceWindowEl {
            day_of_week: self.day_of_week,
            start_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LookerInstanceMaintenanceWindowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstanceMaintenanceWindowElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstanceMaintenanceWindowElRef {
        LookerInstanceMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceMaintenanceWindowElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day_of_week` after provisioning.\nRequired. Day of the week for this MaintenanceWindow (in UTC).\n\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::ListRef<LookerInstanceMaintenanceWindowElStartTimeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstanceOauthConfigEl {
    client_id: terrars::PrimField<std::string::String>,
    client_secret: terrars::PrimField<std::string::String>,
}

impl LookerInstanceOauthConfigEl { }

impl terrars::ToListMappable for LookerInstanceOauthConfigEl {
    type O = terrars::BlockAssignable<LookerInstanceOauthConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstanceOauthConfigEl {
    #[doc = "The client ID for the Oauth config."]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = "The client secret for the Oauth config."]
    pub client_secret: terrars::PrimField<std::string::String>,
}

impl BuildLookerInstanceOauthConfigEl {
    pub fn build(self) -> LookerInstanceOauthConfigEl {
        LookerInstanceOauthConfigEl {
            client_id: self.client_id,
            client_secret: self.client_secret,
        }
    }
}

pub struct LookerInstanceOauthConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstanceOauthConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstanceOauthConfigElRef {
        LookerInstanceOauthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceOauthConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nThe client ID for the Oauth config."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nThe client secret for the Oauth config."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LookerInstanceTimeoutsEl {
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

impl terrars::ToListMappable for LookerInstanceTimeoutsEl {
    type O = terrars::BlockAssignable<LookerInstanceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstanceTimeoutsEl {}

impl BuildLookerInstanceTimeoutsEl {
    pub fn build(self) -> LookerInstanceTimeoutsEl {
        LookerInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct LookerInstanceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstanceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstanceTimeoutsElRef {
        LookerInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceTimeoutsElRef {
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
pub struct LookerInstanceUserMetadataEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_developer_user_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_standard_user_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_viewer_user_count: std::option::Option<terrars::PrimField<f64>>,
}

impl LookerInstanceUserMetadataEl {
    #[doc =
        "Set the field `additional_developer_user_count`.\nNumber of additional Developer Users to allocate to the Looker Instance."]
    pub fn set_additional_developer_user_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.additional_developer_user_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `additional_standard_user_count`.\nNumber of additional Standard Users to allocate to the Looker Instance."]
    pub fn set_additional_standard_user_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.additional_standard_user_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `additional_viewer_user_count`.\nNumber of additional Viewer Users to allocate to the Looker Instance."]
    pub fn set_additional_viewer_user_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.additional_viewer_user_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LookerInstanceUserMetadataEl {
    type O = terrars::BlockAssignable<LookerInstanceUserMetadataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstanceUserMetadataEl {}

impl BuildLookerInstanceUserMetadataEl {
    pub fn build(self) -> LookerInstanceUserMetadataEl {
        LookerInstanceUserMetadataEl {
            additional_developer_user_count: core::default::Default::default(),
            additional_standard_user_count: core::default::Default::default(),
            additional_viewer_user_count: core::default::Default::default(),
        }
    }
}

pub struct LookerInstanceUserMetadataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstanceUserMetadataElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstanceUserMetadataElRef {
        LookerInstanceUserMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceUserMetadataElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additional_developer_user_count` after provisioning.\nNumber of additional Developer Users to allocate to the Looker Instance."]
    pub fn additional_developer_user_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_developer_user_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `additional_standard_user_count` after provisioning.\nNumber of additional Standard Users to allocate to the Looker Instance."]
    pub fn additional_standard_user_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_standard_user_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `additional_viewer_user_count` after provisioning.\nNumber of additional Viewer Users to allocate to the Looker Instance."]
    pub fn additional_viewer_user_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_viewer_user_count", self.base))
    }
}

#[derive(Serialize, Default)]
struct LookerInstanceDynamic {
    admin_settings: std::option::Option<terrars::DynamicBlock<LookerInstanceAdminSettingsEl>>,
    deny_maintenance_period: std::option::Option<terrars::DynamicBlock<LookerInstanceDenyMaintenancePeriodEl>>,
    encryption_config: std::option::Option<terrars::DynamicBlock<LookerInstanceEncryptionConfigEl>>,
    maintenance_window: std::option::Option<terrars::DynamicBlock<LookerInstanceMaintenanceWindowEl>>,
    oauth_config: std::option::Option<terrars::DynamicBlock<LookerInstanceOauthConfigEl>>,
    user_metadata: std::option::Option<terrars::DynamicBlock<LookerInstanceUserMetadataEl>>,
}
