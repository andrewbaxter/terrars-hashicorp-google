use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct LookerInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_edition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_ip_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_ip_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reserved_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_settings: Option<Vec<LookerInstanceAdminSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deny_maintenance_period: Option<Vec<LookerInstanceDenyMaintenancePeriodEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_config: Option<Vec<LookerInstanceEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_window: Option<Vec<LookerInstanceMaintenanceWindowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_config: Option<Vec<LookerInstanceOauthConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<LookerInstanceTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_metadata: Option<Vec<LookerInstanceUserMetadataEl>>,
    dynamic: LookerInstanceDynamic,
}

struct LookerInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LookerInstanceData>,
}

#[derive(Clone)]
pub struct LookerInstance(Rc<LookerInstance_>);

impl LookerInstance {
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

    #[doc= "Set the field `consumer_network`.\nNetwork name in the consumer project in the format of: projects/{project}/global/networks/{network}\nNote that the consumer network may be in a different GCP project than the consumer\nproject that is hosting the Looker Instance."]
    pub fn set_consumer_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().consumer_network = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `platform_edition`.\nPlatform editions for a Looker instance. Each edition maps to a set of instance features, like its size. Must be one of these values:\n- LOOKER_CORE_TRIAL: trial instance\n- LOOKER_CORE_STANDARD: pay as you go standard instance\n- LOOKER_CORE_STANDARD_ANNUAL: subscription standard instance\n- LOOKER_CORE_ENTERPRISE_ANNUAL: subscription enterprise instance\n- LOOKER_CORE_EMBED_ANNUAL: subscription embed instance Default value: \"LOOKER_CORE_TRIAL\" Possible values: [\"LOOKER_CORE_TRIAL\", \"LOOKER_CORE_STANDARD\", \"LOOKER_CORE_STANDARD_ANNUAL\", \"LOOKER_CORE_ENTERPRISE_ANNUAL\", \"LOOKER_CORE_EMBED_ANNUAL\"]"]
    pub fn set_platform_edition(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().platform_edition = Some(v.into());
        self
    }

    #[doc= "Set the field `private_ip_enabled`.\nWhether private IP is enabled on the Looker instance."]
    pub fn set_private_ip_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().private_ip_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `public_ip_enabled`.\nWhether public IP is enabled on the Looker instance."]
    pub fn set_public_ip_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().public_ip_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\nThe name of the Looker region of the instance."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Set the field `reserved_range`.\nName of a reserved IP address range within the consumer network, to be used for\nprivate service access connection. User may or may not specify this in a request."]
    pub fn set_reserved_range(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().reserved_range = Some(v.into());
        self
    }

    #[doc= "Set the field `admin_settings`.\n"]
    pub fn set_admin_settings(self, v: impl Into<BlockAssignable<LookerInstanceAdminSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().admin_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.admin_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `deny_maintenance_period`.\n"]
    pub fn set_deny_maintenance_period(
        self,
        v: impl Into<BlockAssignable<LookerInstanceDenyMaintenancePeriodEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deny_maintenance_period = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deny_maintenance_period = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(self, v: impl Into<BlockAssignable<LookerInstanceEncryptionConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(self, v: impl Into<BlockAssignable<LookerInstanceMaintenanceWindowEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_window = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_window = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `oauth_config`.\n"]
    pub fn set_oauth_config(self, v: impl Into<BlockAssignable<LookerInstanceOauthConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oauth_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oauth_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<LookerInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Set the field `user_metadata`.\n"]
    pub fn set_user_metadata(self, v: impl Into<BlockAssignable<LookerInstanceUserMetadataEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_metadata = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_metadata = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `consumer_network` after provisioning.\nNetwork name in the consumer project in the format of: projects/{project}/global/networks/{network}\nNote that the consumer network may be in a different GCP project than the consumer\nproject that is hosting the Looker Instance."]
    pub fn consumer_network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.consumer_network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nThe time the instance was created in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `egress_public_ip` after provisioning.\nPublic Egress IP (IPv4)."]
    pub fn egress_public_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.egress_public_ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ingress_private_ip` after provisioning.\nPrivate Ingress IP (IPv4)."]
    pub fn ingress_private_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ingress_private_ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ingress_public_ip` after provisioning.\nPublic Ingress IP (IPv4)."]
    pub fn ingress_public_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ingress_public_ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `looker_uri` after provisioning.\nLooker instance URI which can be used to access the Looker Instance UI."]
    pub fn looker_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.looker_uri", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `looker_version` after provisioning.\nThe Looker version that the instance is using."]
    pub fn looker_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.looker_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe ID of the instance or a fully qualified identifier for the instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `platform_edition` after provisioning.\nPlatform editions for a Looker instance. Each edition maps to a set of instance features, like its size. Must be one of these values:\n- LOOKER_CORE_TRIAL: trial instance\n- LOOKER_CORE_STANDARD: pay as you go standard instance\n- LOOKER_CORE_STANDARD_ANNUAL: subscription standard instance\n- LOOKER_CORE_ENTERPRISE_ANNUAL: subscription enterprise instance\n- LOOKER_CORE_EMBED_ANNUAL: subscription embed instance Default value: \"LOOKER_CORE_TRIAL\" Possible values: [\"LOOKER_CORE_TRIAL\", \"LOOKER_CORE_STANDARD\", \"LOOKER_CORE_STANDARD_ANNUAL\", \"LOOKER_CORE_ENTERPRISE_ANNUAL\", \"LOOKER_CORE_EMBED_ANNUAL\"]"]
    pub fn platform_edition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_edition", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_ip_enabled` after provisioning.\nWhether private IP is enabled on the Looker instance."]
    pub fn private_ip_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_ip_enabled` after provisioning.\nWhether public IP is enabled on the Looker instance."]
    pub fn public_ip_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ip_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe name of the Looker region of the instance."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reserved_range` after provisioning.\nName of a reserved IP address range within the consumer network, to be used for\nprivate service access connection. User may or may not specify this in a request."]
    pub fn reserved_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reserved_range", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_time` after provisioning.\nThe time the instance was updated in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `admin_settings` after provisioning.\n"]
    pub fn admin_settings(&self) -> ListRef<LookerInstanceAdminSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.admin_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deny_maintenance_period` after provisioning.\n"]
    pub fn deny_maintenance_period(&self) -> ListRef<LookerInstanceDenyMaintenancePeriodElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deny_maintenance_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> ListRef<LookerInstanceEncryptionConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> ListRef<LookerInstanceMaintenanceWindowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(&self) -> ListRef<LookerInstanceOauthConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LookerInstanceTimeoutsElRef {
        LookerInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_metadata` after provisioning.\n"]
    pub fn user_metadata(&self) -> ListRef<LookerInstanceUserMetadataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_metadata", self.extract_ref()))
    }
}

impl Referable for LookerInstance {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for LookerInstance { }

impl ToListMappable for LookerInstance {
    type O = ListRef<LookerInstanceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LookerInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_looker_instance".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLookerInstance {
    pub tf_id: String,
    #[doc= "The ID of the instance or a fully qualified identifier for the instance."]
    pub name: PrimField<String>,
}

impl BuildLookerInstance {
    pub fn build(self, stack: &mut Stack) -> LookerInstance {
        let out = LookerInstance(Rc::new(LookerInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LookerInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LookerInstanceRef {
    shared: StackShared,
    base: String,
}

impl Ref for LookerInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LookerInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `consumer_network` after provisioning.\nNetwork name in the consumer project in the format of: projects/{project}/global/networks/{network}\nNote that the consumer network may be in a different GCP project than the consumer\nproject that is hosting the Looker Instance."]
    pub fn consumer_network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.consumer_network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nThe time the instance was created in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `egress_public_ip` after provisioning.\nPublic Egress IP (IPv4)."]
    pub fn egress_public_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.egress_public_ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ingress_private_ip` after provisioning.\nPrivate Ingress IP (IPv4)."]
    pub fn ingress_private_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ingress_private_ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ingress_public_ip` after provisioning.\nPublic Ingress IP (IPv4)."]
    pub fn ingress_public_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ingress_public_ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `looker_uri` after provisioning.\nLooker instance URI which can be used to access the Looker Instance UI."]
    pub fn looker_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.looker_uri", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `looker_version` after provisioning.\nThe Looker version that the instance is using."]
    pub fn looker_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.looker_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe ID of the instance or a fully qualified identifier for the instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `platform_edition` after provisioning.\nPlatform editions for a Looker instance. Each edition maps to a set of instance features, like its size. Must be one of these values:\n- LOOKER_CORE_TRIAL: trial instance\n- LOOKER_CORE_STANDARD: pay as you go standard instance\n- LOOKER_CORE_STANDARD_ANNUAL: subscription standard instance\n- LOOKER_CORE_ENTERPRISE_ANNUAL: subscription enterprise instance\n- LOOKER_CORE_EMBED_ANNUAL: subscription embed instance Default value: \"LOOKER_CORE_TRIAL\" Possible values: [\"LOOKER_CORE_TRIAL\", \"LOOKER_CORE_STANDARD\", \"LOOKER_CORE_STANDARD_ANNUAL\", \"LOOKER_CORE_ENTERPRISE_ANNUAL\", \"LOOKER_CORE_EMBED_ANNUAL\"]"]
    pub fn platform_edition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_edition", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_ip_enabled` after provisioning.\nWhether private IP is enabled on the Looker instance."]
    pub fn private_ip_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_ip_enabled` after provisioning.\nWhether public IP is enabled on the Looker instance."]
    pub fn public_ip_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ip_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe name of the Looker region of the instance."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reserved_range` after provisioning.\nName of a reserved IP address range within the consumer network, to be used for\nprivate service access connection. User may or may not specify this in a request."]
    pub fn reserved_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reserved_range", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_time` after provisioning.\nThe time the instance was updated in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `admin_settings` after provisioning.\n"]
    pub fn admin_settings(&self) -> ListRef<LookerInstanceAdminSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.admin_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deny_maintenance_period` after provisioning.\n"]
    pub fn deny_maintenance_period(&self) -> ListRef<LookerInstanceDenyMaintenancePeriodElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deny_maintenance_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> ListRef<LookerInstanceEncryptionConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> ListRef<LookerInstanceMaintenanceWindowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(&self) -> ListRef<LookerInstanceOauthConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LookerInstanceTimeoutsElRef {
        LookerInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_metadata` after provisioning.\n"]
    pub fn user_metadata(&self) -> ListRef<LookerInstanceUserMetadataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_metadata", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LookerInstanceAdminSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_email_domains: Option<ListField<PrimField<String>>>,
}

impl LookerInstanceAdminSettingsEl {
    #[doc= "Set the field `allowed_email_domains`.\nEmail domain allowlist for the instance.\n\nDefine the email domains to which your users can deliver Looker (Google Cloud core) content.\nUpdating this list will restart the instance. Updating the allowed email domains from terraform\nmeans the value provided will be considered as the entire list and not an amendment to the\nexisting list of allowed email domains."]
    pub fn set_allowed_email_domains(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_email_domains = Some(v.into());
        self
    }
}

impl ToListMappable for LookerInstanceAdminSettingsEl {
    type O = BlockAssignable<LookerInstanceAdminSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for LookerInstanceAdminSettingsElRef {
    fn new(shared: StackShared, base: String) -> LookerInstanceAdminSettingsElRef {
        LookerInstanceAdminSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceAdminSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allowed_email_domains` after provisioning.\nEmail domain allowlist for the instance.\n\nDefine the email domains to which your users can deliver Looker (Google Cloud core) content.\nUpdating this list will restart the instance. Updating the allowed email domains from terraform\nmeans the value provided will be considered as the entire list and not an amendment to the\nexisting list of allowed email domains."]
    pub fn allowed_email_domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_email_domains", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstanceDenyMaintenancePeriodElEndDateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<PrimField<f64>>,
}

impl LookerInstanceDenyMaintenancePeriodElEndDateEl {
    #[doc= "Set the field `day`.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't significant."]
    pub fn set_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.day = Some(v.into());
        self
    }

    #[doc= "Set the field `month`.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn set_month(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.month = Some(v.into());
        self
    }

    #[doc= "Set the field `year`.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn set_year(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.year = Some(v.into());
        self
    }
}

impl ToListMappable for LookerInstanceDenyMaintenancePeriodElEndDateEl {
    type O = BlockAssignable<LookerInstanceDenyMaintenancePeriodElEndDateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for LookerInstanceDenyMaintenancePeriodElEndDateElRef {
    fn new(shared: StackShared, base: String) -> LookerInstanceDenyMaintenancePeriodElEndDateElRef {
        LookerInstanceDenyMaintenancePeriodElEndDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceDenyMaintenancePeriodElEndDateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't significant."]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc= "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc= "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn year(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstanceDenyMaintenancePeriodElStartDateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<PrimField<f64>>,
}

impl LookerInstanceDenyMaintenancePeriodElStartDateEl {
    #[doc= "Set the field `day`.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't significant."]
    pub fn set_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.day = Some(v.into());
        self
    }

    #[doc= "Set the field `month`.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn set_month(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.month = Some(v.into());
        self
    }

    #[doc= "Set the field `year`.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn set_year(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.year = Some(v.into());
        self
    }
}

impl ToListMappable for LookerInstanceDenyMaintenancePeriodElStartDateEl {
    type O = BlockAssignable<LookerInstanceDenyMaintenancePeriodElStartDateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for LookerInstanceDenyMaintenancePeriodElStartDateElRef {
    fn new(shared: StackShared, base: String) -> LookerInstanceDenyMaintenancePeriodElStartDateElRef {
        LookerInstanceDenyMaintenancePeriodElStartDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceDenyMaintenancePeriodElStartDateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't significant."]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc= "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc= "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn year(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstanceDenyMaintenancePeriodElTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}

impl LookerInstanceDenyMaintenancePeriodElTimeEl {
    #[doc= "Set the field `hours`.\nHours of day in 24 hour format. Should be from 0 to 23."]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }

    #[doc= "Set the field `minutes`.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }

    #[doc= "Set the field `nanos`.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }

    #[doc= "Set the field `seconds`.\nSeconds of minutes of the time. Must normally be from 0 to 59."]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}

impl ToListMappable for LookerInstanceDenyMaintenancePeriodElTimeEl {
    type O = BlockAssignable<LookerInstanceDenyMaintenancePeriodElTimeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for LookerInstanceDenyMaintenancePeriodElTimeElRef {
    fn new(shared: StackShared, base: String) -> LookerInstanceDenyMaintenancePeriodElTimeElRef {
        LookerInstanceDenyMaintenancePeriodElTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceDenyMaintenancePeriodElTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `hours` after provisioning.\nHours of day in 24 hour format. Should be from 0 to 23."]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc= "Get a reference to the value of field `minutes` after provisioning.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc= "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc= "Get a reference to the value of field `seconds` after provisioning.\nSeconds of minutes of the time. Must normally be from 0 to 59."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct LookerInstanceDenyMaintenancePeriodElDynamic {
    end_date: Option<DynamicBlock<LookerInstanceDenyMaintenancePeriodElEndDateEl>>,
    start_date: Option<DynamicBlock<LookerInstanceDenyMaintenancePeriodElStartDateEl>>,
    time: Option<DynamicBlock<LookerInstanceDenyMaintenancePeriodElTimeEl>>,
}

#[derive(Serialize)]
pub struct LookerInstanceDenyMaintenancePeriodEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<Vec<LookerInstanceDenyMaintenancePeriodElEndDateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<Vec<LookerInstanceDenyMaintenancePeriodElStartDateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<Vec<LookerInstanceDenyMaintenancePeriodElTimeEl>>,
    dynamic: LookerInstanceDenyMaintenancePeriodElDynamic,
}

impl LookerInstanceDenyMaintenancePeriodEl {
    #[doc= "Set the field `end_date`.\n"]
    pub fn set_end_date(
        mut self,
        v: impl Into<BlockAssignable<LookerInstanceDenyMaintenancePeriodElEndDateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.end_date = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.end_date = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `start_date`.\n"]
    pub fn set_start_date(
        mut self,
        v: impl Into<BlockAssignable<LookerInstanceDenyMaintenancePeriodElStartDateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.start_date = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.start_date = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `time`.\n"]
    pub fn set_time(mut self, v: impl Into<BlockAssignable<LookerInstanceDenyMaintenancePeriodElTimeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.time = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.time = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for LookerInstanceDenyMaintenancePeriodEl {
    type O = BlockAssignable<LookerInstanceDenyMaintenancePeriodEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct LookerInstanceDenyMaintenancePeriodElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LookerInstanceDenyMaintenancePeriodElRef {
    fn new(shared: StackShared, base: String) -> LookerInstanceDenyMaintenancePeriodElRef {
        LookerInstanceDenyMaintenancePeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceDenyMaintenancePeriodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> ListRef<LookerInstanceDenyMaintenancePeriodElEndDateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.end_date", self.base))
    }

    #[doc= "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> ListRef<LookerInstanceDenyMaintenancePeriodElStartDateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.start_date", self.base))
    }

    #[doc= "Get a reference to the value of field `time` after provisioning.\n"]
    pub fn time(&self) -> ListRef<LookerInstanceDenyMaintenancePeriodElTimeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstanceEncryptionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
}

impl LookerInstanceEncryptionConfigEl {
    #[doc= "Set the field `kms_key_name`.\nName of the customer managed encryption key (CMEK) in KMS."]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
}

impl ToListMappable for LookerInstanceEncryptionConfigEl {
    type O = BlockAssignable<LookerInstanceEncryptionConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for LookerInstanceEncryptionConfigElRef {
    fn new(shared: StackShared, base: String) -> LookerInstanceEncryptionConfigElRef {
        LookerInstanceEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `kms_key_name` after provisioning.\nName of the customer managed encryption key (CMEK) in KMS."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc= "Get a reference to the value of field `kms_key_name_version` after provisioning.\nFull name and version of the CMEK key currently in use to encrypt Looker data."]
    pub fn kms_key_name_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name_version", self.base))
    }

    #[doc= "Get a reference to the value of field `kms_key_state` after provisioning.\nStatus of the customer managed encryption key (CMEK) in KMS."]
    pub fn kms_key_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_state", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstanceMaintenanceWindowElStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}

impl LookerInstanceMaintenanceWindowElStartTimeEl {
    #[doc= "Set the field `hours`.\nHours of day in 24 hour format. Should be from 0 to 23."]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }

    #[doc= "Set the field `minutes`.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }

    #[doc= "Set the field `nanos`.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }

    #[doc= "Set the field `seconds`.\nSeconds of minutes of the time. Must normally be from 0 to 59."]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}

impl ToListMappable for LookerInstanceMaintenanceWindowElStartTimeEl {
    type O = BlockAssignable<LookerInstanceMaintenanceWindowElStartTimeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for LookerInstanceMaintenanceWindowElStartTimeElRef {
    fn new(shared: StackShared, base: String) -> LookerInstanceMaintenanceWindowElStartTimeElRef {
        LookerInstanceMaintenanceWindowElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceMaintenanceWindowElStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `hours` after provisioning.\nHours of day in 24 hour format. Should be from 0 to 23."]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc= "Get a reference to the value of field `minutes` after provisioning.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc= "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc= "Get a reference to the value of field `seconds` after provisioning.\nSeconds of minutes of the time. Must normally be from 0 to 59."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct LookerInstanceMaintenanceWindowElDynamic {
    start_time: Option<DynamicBlock<LookerInstanceMaintenanceWindowElStartTimeEl>>,
}

#[derive(Serialize)]
pub struct LookerInstanceMaintenanceWindowEl {
    day_of_week: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<Vec<LookerInstanceMaintenanceWindowElStartTimeEl>>,
    dynamic: LookerInstanceMaintenanceWindowElDynamic,
}

impl LookerInstanceMaintenanceWindowEl {
    #[doc= "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl Into<BlockAssignable<LookerInstanceMaintenanceWindowElStartTimeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.start_time = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for LookerInstanceMaintenanceWindowEl {
    type O = BlockAssignable<LookerInstanceMaintenanceWindowEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstanceMaintenanceWindowEl {
    #[doc= "Required. Day of the week for this MaintenanceWindow (in UTC).\n\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day_of_week: PrimField<String>,
}

impl BuildLookerInstanceMaintenanceWindowEl {
    pub fn build(self) -> LookerInstanceMaintenanceWindowEl {
        LookerInstanceMaintenanceWindowEl {
            day_of_week: self.day_of_week,
            start_time: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LookerInstanceMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LookerInstanceMaintenanceWindowElRef {
    fn new(shared: StackShared, base: String) -> LookerInstanceMaintenanceWindowElRef {
        LookerInstanceMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `day_of_week` after provisioning.\nRequired. Day of the week for this MaintenanceWindow (in UTC).\n\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day_of_week(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc= "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> ListRef<LookerInstanceMaintenanceWindowElStartTimeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstanceOauthConfigEl {
    client_id: PrimField<String>,
    client_secret: PrimField<String>,
}

impl LookerInstanceOauthConfigEl { }

impl ToListMappable for LookerInstanceOauthConfigEl {
    type O = BlockAssignable<LookerInstanceOauthConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstanceOauthConfigEl {
    #[doc= "The client ID for the Oauth config."]
    pub client_id: PrimField<String>,
    #[doc= "The client secret for the Oauth config."]
    pub client_secret: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for LookerInstanceOauthConfigElRef {
    fn new(shared: StackShared, base: String) -> LookerInstanceOauthConfigElRef {
        LookerInstanceOauthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceOauthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `client_id` after provisioning.\nThe client ID for the Oauth config."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc= "Get a reference to the value of field `client_secret` after provisioning.\nThe client secret for the Oauth config."]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl LookerInstanceTimeoutsEl {
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

impl ToListMappable for LookerInstanceTimeoutsEl {
    type O = BlockAssignable<LookerInstanceTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for LookerInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> LookerInstanceTimeoutsElRef {
        LookerInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceTimeoutsElRef {
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

#[derive(Serialize)]
pub struct LookerInstanceUserMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_developer_user_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_standard_user_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_viewer_user_count: Option<PrimField<f64>>,
}

impl LookerInstanceUserMetadataEl {
    #[doc= "Set the field `additional_developer_user_count`.\nNumber of additional Developer Users to allocate to the Looker Instance."]
    pub fn set_additional_developer_user_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.additional_developer_user_count = Some(v.into());
        self
    }

    #[doc= "Set the field `additional_standard_user_count`.\nNumber of additional Standard Users to allocate to the Looker Instance."]
    pub fn set_additional_standard_user_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.additional_standard_user_count = Some(v.into());
        self
    }

    #[doc= "Set the field `additional_viewer_user_count`.\nNumber of additional Viewer Users to allocate to the Looker Instance."]
    pub fn set_additional_viewer_user_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.additional_viewer_user_count = Some(v.into());
        self
    }
}

impl ToListMappable for LookerInstanceUserMetadataEl {
    type O = BlockAssignable<LookerInstanceUserMetadataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for LookerInstanceUserMetadataElRef {
    fn new(shared: StackShared, base: String) -> LookerInstanceUserMetadataElRef {
        LookerInstanceUserMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstanceUserMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `additional_developer_user_count` after provisioning.\nNumber of additional Developer Users to allocate to the Looker Instance."]
    pub fn additional_developer_user_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.additional_developer_user_count", self.base))
    }

    #[doc= "Get a reference to the value of field `additional_standard_user_count` after provisioning.\nNumber of additional Standard Users to allocate to the Looker Instance."]
    pub fn additional_standard_user_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.additional_standard_user_count", self.base))
    }

    #[doc= "Get a reference to the value of field `additional_viewer_user_count` after provisioning.\nNumber of additional Viewer Users to allocate to the Looker Instance."]
    pub fn additional_viewer_user_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.additional_viewer_user_count", self.base))
    }
}

#[derive(Serialize, Default)]
struct LookerInstanceDynamic {
    admin_settings: Option<DynamicBlock<LookerInstanceAdminSettingsEl>>,
    deny_maintenance_period: Option<DynamicBlock<LookerInstanceDenyMaintenancePeriodEl>>,
    encryption_config: Option<DynamicBlock<LookerInstanceEncryptionConfigEl>>,
    maintenance_window: Option<DynamicBlock<LookerInstanceMaintenanceWindowEl>>,
    oauth_config: Option<DynamicBlock<LookerInstanceOauthConfigEl>>,
    user_metadata: Option<DynamicBlock<LookerInstanceUserMetadataEl>>,
}
