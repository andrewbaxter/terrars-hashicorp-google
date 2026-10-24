use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AssuredWorkloadsWorkloadData {
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
    billing_account: std::option::Option<terrars::PrimField<std::string::String>>,
    compliance_regime: terrars::PrimField<std::string::String>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_sovereign_controls: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    organization: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_resources_parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    violation_notifications_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_settings: std::option::Option<std::vec::Vec<AssuredWorkloadsWorkloadKmsSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partner_permissions: std::option::Option<std::vec::Vec<AssuredWorkloadsWorkloadPartnerPermissionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_settings: std::option::Option<std::vec::Vec<AssuredWorkloadsWorkloadResourceSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AssuredWorkloadsWorkloadTimeoutsEl>,
    dynamic: AssuredWorkloadsWorkloadDynamic,
}

struct AssuredWorkloadsWorkload_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AssuredWorkloadsWorkloadData>,
}

#[derive(Clone)]
pub struct AssuredWorkloadsWorkload(std::rc::Rc<AssuredWorkloadsWorkload_>);

impl AssuredWorkloadsWorkload {
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
        "Set the field `billing_account`.\nOptional. Input only. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`."]
    pub fn set_billing_account(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().billing_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_sovereign_controls`.\nOptional. Indicates the sovereignty status of the given workload. Currently meant to be used by Europe/Canada customers."]
    pub fn set_enable_sovereign_controls(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_sovereign_controls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nOptional. Labels applied to the workload.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `partner`.\nOptional. Partner regime associated with this workload. Possible values: PARTNER_UNSPECIFIED, LOCAL_CONTROLS_BY_S3NS, SOVEREIGN_CONTROLS_BY_T_SYSTEMS, SOVEREIGN_CONTROLS_BY_SIA_MINSAIT, SOVEREIGN_CONTROLS_BY_PSN"]
    pub fn set_partner(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().partner = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `provisioned_resources_parent`.\nInput only. The parent resource for the resources managed by this Assured Workload. May be either empty or a folder resource which is a child of the Workload parent. If not specified all resources are created under the parent organization. Format: folders/{folder_id}"]
    pub fn set_provisioned_resources_parent(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().provisioned_resources_parent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `violation_notifications_enabled`.\nOptional. Indicates whether the e-mail notification for a violation is enabled for a workload. This value will be by default True, and if not present will be considered as true. This should only be updated via updateWorkload call. Any Changes to this field during the createWorkload call will not be honored. This will always be true while creating the workload."]
    pub fn set_violation_notifications_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().violation_notifications_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_settings`.\n"]
    pub fn set_kms_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AssuredWorkloadsWorkloadKmsSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kms_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kms_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `partner_permissions`.\n"]
    pub fn set_partner_permissions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AssuredWorkloadsWorkloadPartnerPermissionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().partner_permissions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.partner_permissions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_settings`.\n"]
    pub fn set_resource_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AssuredWorkloadsWorkloadResourceSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AssuredWorkloadsWorkloadTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `billing_account` after provisioning.\nOptional. Input only. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`."]
    pub fn billing_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compliance_regime` after provisioning.\nRequired. Immutable. Compliance Regime associated with this workload. Possible values: COMPLIANCE_REGIME_UNSPECIFIED, IL4, CJIS, FEDRAMP_HIGH, FEDRAMP_MODERATE, US_REGIONAL_ACCESS, HIPAA, HITRUST, EU_REGIONS_AND_SUPPORT, CA_REGIONS_AND_SUPPORT, ITAR, AU_REGIONS_AND_US_SUPPORT, ASSURED_WORKLOADS_FOR_PARTNERS, ISR_REGIONS, ISR_REGIONS_AND_SUPPORT, CA_PROTECTED_B, IL5, IL2, JP_REGIONS_AND_SUPPORT"]
    pub fn compliance_regime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compliance_regime", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compliance_status` after provisioning.\nOutput only. Count of active Violations in the Workload."]
    pub fn compliance_status(&self) -> terrars::ListRef<AssuredWorkloadsWorkloadComplianceStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compliance_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compliant_but_disallowed_services` after provisioning.\nOutput only. Urls for services which are compliant for this Assured Workload, but which are currently disallowed by the ResourceUsageRestriction org policy. Invoke workloads.restrictAllowedResources endpoint to allow your project developers to use these services in their environment."]
    pub fn compliant_but_disallowed_services(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.compliant_but_disallowed_services", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Immutable. The Workload creation timestamp."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. The user-assigned display name of the Workload. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, and spaces. Example: My Workload"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ekm_provisioning_response` after provisioning.\nOptional. Represents the Ekm Provisioning State of the given workload."]
    pub fn ekm_provisioning_response(&self) -> terrars::ListRef<AssuredWorkloadsWorkloadEkmProvisioningResponseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ekm_provisioning_response", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_sovereign_controls` after provisioning.\nOptional. Indicates the sovereignty status of the given workload. Currently meant to be used by Europe/Canada customers."]
    pub fn enable_sovereign_controls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_sovereign_controls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kaj_enrollment_state` after provisioning.\nOutput only. Represents the KAJ enrollment state of the given workload. Possible values: KAJ_ENROLLMENT_STATE_UNSPECIFIED, KAJ_ENROLLMENT_STATE_PENDING, KAJ_ENROLLMENT_STATE_COMPLETE"]
    pub fn kaj_enrollment_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kaj_enrollment_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels applied to the workload.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. The resource name of the workload."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization` after provisioning.\nThe organization for the resource"]
    pub fn organization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `partner` after provisioning.\nOptional. Partner regime associated with this workload. Possible values: PARTNER_UNSPECIFIED, LOCAL_CONTROLS_BY_S3NS, SOVEREIGN_CONTROLS_BY_T_SYSTEMS, SOVEREIGN_CONTROLS_BY_SIA_MINSAIT, SOVEREIGN_CONTROLS_BY_PSN"]
    pub fn partner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partner", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provisioned_resources_parent` after provisioning.\nInput only. The parent resource for the resources managed by this Assured Workload. May be either empty or a folder resource which is a child of the Workload parent. If not specified all resources are created under the parent organization. Format: folders/{folder_id}"]
    pub fn provisioned_resources_parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_resources_parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nOutput only. The resources associated with this workload. These resources will be created when creating the workload. If any of the projects already exist, the workload creation will fail. Always read only."]
    pub fn resources(&self) -> terrars::ListRef<AssuredWorkloadsWorkloadResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `saa_enrollment_response` after provisioning.\nOutput only. Represents the SAA enrollment response of the given workload. SAA enrollment response is queried during workloads.get call. In failure cases, user friendly error message is shown in SAA details page."]
    pub fn saa_enrollment_response(&self) -> terrars::ListRef<AssuredWorkloadsWorkloadSaaEnrollmentResponseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saa_enrollment_response", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `violation_notifications_enabled` after provisioning.\nOptional. Indicates whether the e-mail notification for a violation is enabled for a workload. This value will be by default True, and if not present will be considered as true. This should only be updated via updateWorkload call. Any Changes to this field during the createWorkload call will not be honored. This will always be true while creating the workload."]
    pub fn violation_notifications_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.violation_notifications_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kms_settings` after provisioning.\n"]
    pub fn kms_settings(&self) -> terrars::ListRef<AssuredWorkloadsWorkloadKmsSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kms_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partner_permissions` after provisioning.\n"]
    pub fn partner_permissions(&self) -> terrars::ListRef<AssuredWorkloadsWorkloadPartnerPermissionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partner_permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_settings` after provisioning.\n"]
    pub fn resource_settings(&self) -> terrars::ListRef<AssuredWorkloadsWorkloadResourceSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AssuredWorkloadsWorkloadTimeoutsElRef {
        AssuredWorkloadsWorkloadTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for AssuredWorkloadsWorkload {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AssuredWorkloadsWorkload { }

impl terrars::ToListMappable for AssuredWorkloadsWorkload {
    type O = terrars::ListRef<AssuredWorkloadsWorkloadRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AssuredWorkloadsWorkload_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_assured_workloads_workload".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAssuredWorkloadsWorkload {
    pub tf_id: std::string::String,
    #[doc =
        "Required. Immutable. Compliance Regime associated with this workload. Possible values: COMPLIANCE_REGIME_UNSPECIFIED, IL4, CJIS, FEDRAMP_HIGH, FEDRAMP_MODERATE, US_REGIONAL_ACCESS, HIPAA, HITRUST, EU_REGIONS_AND_SUPPORT, CA_REGIONS_AND_SUPPORT, ITAR, AU_REGIONS_AND_US_SUPPORT, ASSURED_WORKLOADS_FOR_PARTNERS, ISR_REGIONS, ISR_REGIONS_AND_SUPPORT, CA_PROTECTED_B, IL5, IL2, JP_REGIONS_AND_SUPPORT"]
    pub compliance_regime: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. The user-assigned display name of the Workload. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, and spaces. Example: My Workload"]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The organization for the resource"]
    pub organization: terrars::PrimField<std::string::String>,
}

impl BuildAssuredWorkloadsWorkload {
    pub fn build(self, stack: &mut terrars::Stack) -> AssuredWorkloadsWorkload {
        let out = AssuredWorkloadsWorkload(std::rc::Rc::new(AssuredWorkloadsWorkload_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AssuredWorkloadsWorkloadData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                billing_account: core::default::Default::default(),
                compliance_regime: self.compliance_regime,
                display_name: self.display_name,
                enable_sovereign_controls: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                organization: self.organization,
                partner: core::default::Default::default(),
                provisioned_resources_parent: core::default::Default::default(),
                violation_notifications_enabled: core::default::Default::default(),
                kms_settings: core::default::Default::default(),
                partner_permissions: core::default::Default::default(),
                resource_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AssuredWorkloadsWorkloadRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AssuredWorkloadsWorkloadRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AssuredWorkloadsWorkloadRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `billing_account` after provisioning.\nOptional. Input only. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`."]
    pub fn billing_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compliance_regime` after provisioning.\nRequired. Immutable. Compliance Regime associated with this workload. Possible values: COMPLIANCE_REGIME_UNSPECIFIED, IL4, CJIS, FEDRAMP_HIGH, FEDRAMP_MODERATE, US_REGIONAL_ACCESS, HIPAA, HITRUST, EU_REGIONS_AND_SUPPORT, CA_REGIONS_AND_SUPPORT, ITAR, AU_REGIONS_AND_US_SUPPORT, ASSURED_WORKLOADS_FOR_PARTNERS, ISR_REGIONS, ISR_REGIONS_AND_SUPPORT, CA_PROTECTED_B, IL5, IL2, JP_REGIONS_AND_SUPPORT"]
    pub fn compliance_regime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compliance_regime", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compliance_status` after provisioning.\nOutput only. Count of active Violations in the Workload."]
    pub fn compliance_status(&self) -> terrars::ListRef<AssuredWorkloadsWorkloadComplianceStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compliance_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compliant_but_disallowed_services` after provisioning.\nOutput only. Urls for services which are compliant for this Assured Workload, but which are currently disallowed by the ResourceUsageRestriction org policy. Invoke workloads.restrictAllowedResources endpoint to allow your project developers to use these services in their environment."]
    pub fn compliant_but_disallowed_services(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.compliant_but_disallowed_services", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Immutable. The Workload creation timestamp."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. The user-assigned display name of the Workload. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, and spaces. Example: My Workload"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ekm_provisioning_response` after provisioning.\nOptional. Represents the Ekm Provisioning State of the given workload."]
    pub fn ekm_provisioning_response(&self) -> terrars::ListRef<AssuredWorkloadsWorkloadEkmProvisioningResponseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ekm_provisioning_response", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_sovereign_controls` after provisioning.\nOptional. Indicates the sovereignty status of the given workload. Currently meant to be used by Europe/Canada customers."]
    pub fn enable_sovereign_controls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_sovereign_controls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kaj_enrollment_state` after provisioning.\nOutput only. Represents the KAJ enrollment state of the given workload. Possible values: KAJ_ENROLLMENT_STATE_UNSPECIFIED, KAJ_ENROLLMENT_STATE_PENDING, KAJ_ENROLLMENT_STATE_COMPLETE"]
    pub fn kaj_enrollment_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kaj_enrollment_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels applied to the workload.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. The resource name of the workload."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization` after provisioning.\nThe organization for the resource"]
    pub fn organization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `partner` after provisioning.\nOptional. Partner regime associated with this workload. Possible values: PARTNER_UNSPECIFIED, LOCAL_CONTROLS_BY_S3NS, SOVEREIGN_CONTROLS_BY_T_SYSTEMS, SOVEREIGN_CONTROLS_BY_SIA_MINSAIT, SOVEREIGN_CONTROLS_BY_PSN"]
    pub fn partner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partner", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provisioned_resources_parent` after provisioning.\nInput only. The parent resource for the resources managed by this Assured Workload. May be either empty or a folder resource which is a child of the Workload parent. If not specified all resources are created under the parent organization. Format: folders/{folder_id}"]
    pub fn provisioned_resources_parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_resources_parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nOutput only. The resources associated with this workload. These resources will be created when creating the workload. If any of the projects already exist, the workload creation will fail. Always read only."]
    pub fn resources(&self) -> terrars::ListRef<AssuredWorkloadsWorkloadResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `saa_enrollment_response` after provisioning.\nOutput only. Represents the SAA enrollment response of the given workload. SAA enrollment response is queried during workloads.get call. In failure cases, user friendly error message is shown in SAA details page."]
    pub fn saa_enrollment_response(&self) -> terrars::ListRef<AssuredWorkloadsWorkloadSaaEnrollmentResponseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saa_enrollment_response", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `violation_notifications_enabled` after provisioning.\nOptional. Indicates whether the e-mail notification for a violation is enabled for a workload. This value will be by default True, and if not present will be considered as true. This should only be updated via updateWorkload call. Any Changes to this field during the createWorkload call will not be honored. This will always be true while creating the workload."]
    pub fn violation_notifications_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.violation_notifications_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kms_settings` after provisioning.\n"]
    pub fn kms_settings(&self) -> terrars::ListRef<AssuredWorkloadsWorkloadKmsSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kms_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partner_permissions` after provisioning.\n"]
    pub fn partner_permissions(&self) -> terrars::ListRef<AssuredWorkloadsWorkloadPartnerPermissionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partner_permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_settings` after provisioning.\n"]
    pub fn resource_settings(&self) -> terrars::ListRef<AssuredWorkloadsWorkloadResourceSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AssuredWorkloadsWorkloadTimeoutsElRef {
        AssuredWorkloadsWorkloadTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AssuredWorkloadsWorkloadComplianceStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acknowledged_violation_count: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active_violation_count: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
}

impl AssuredWorkloadsWorkloadComplianceStatusEl {
    #[doc = "Set the field `acknowledged_violation_count`.\n"]
    pub fn set_acknowledged_violation_count(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.acknowledged_violation_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `active_violation_count`.\n"]
    pub fn set_active_violation_count(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.active_violation_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AssuredWorkloadsWorkloadComplianceStatusEl {
    type O = terrars::BlockAssignable<AssuredWorkloadsWorkloadComplianceStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAssuredWorkloadsWorkloadComplianceStatusEl {}

impl BuildAssuredWorkloadsWorkloadComplianceStatusEl {
    pub fn build(self) -> AssuredWorkloadsWorkloadComplianceStatusEl {
        AssuredWorkloadsWorkloadComplianceStatusEl {
            acknowledged_violation_count: core::default::Default::default(),
            active_violation_count: core::default::Default::default(),
        }
    }
}

pub struct AssuredWorkloadsWorkloadComplianceStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AssuredWorkloadsWorkloadComplianceStatusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AssuredWorkloadsWorkloadComplianceStatusElRef {
        AssuredWorkloadsWorkloadComplianceStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AssuredWorkloadsWorkloadComplianceStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acknowledged_violation_count` after provisioning.\n"]
    pub fn acknowledged_violation_count(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acknowledged_violation_count", self.base))
    }

    #[doc = "Get a reference to the value of field `active_violation_count` after provisioning.\n"]
    pub fn active_violation_count(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.active_violation_count", self.base))
    }
}

#[derive(Serialize)]
pub struct AssuredWorkloadsWorkloadEkmProvisioningResponseEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ekm_provisioning_error_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ekm_provisioning_error_mapping: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ekm_provisioning_state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AssuredWorkloadsWorkloadEkmProvisioningResponseEl {
    #[doc = "Set the field `ekm_provisioning_error_domain`.\n"]
    pub fn set_ekm_provisioning_error_domain(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ekm_provisioning_error_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ekm_provisioning_error_mapping`.\n"]
    pub fn set_ekm_provisioning_error_mapping(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ekm_provisioning_error_mapping = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ekm_provisioning_state`.\n"]
    pub fn set_ekm_provisioning_state(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ekm_provisioning_state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AssuredWorkloadsWorkloadEkmProvisioningResponseEl {
    type O = terrars::BlockAssignable<AssuredWorkloadsWorkloadEkmProvisioningResponseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAssuredWorkloadsWorkloadEkmProvisioningResponseEl {}

impl BuildAssuredWorkloadsWorkloadEkmProvisioningResponseEl {
    pub fn build(self) -> AssuredWorkloadsWorkloadEkmProvisioningResponseEl {
        AssuredWorkloadsWorkloadEkmProvisioningResponseEl {
            ekm_provisioning_error_domain: core::default::Default::default(),
            ekm_provisioning_error_mapping: core::default::Default::default(),
            ekm_provisioning_state: core::default::Default::default(),
        }
    }
}

pub struct AssuredWorkloadsWorkloadEkmProvisioningResponseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AssuredWorkloadsWorkloadEkmProvisioningResponseElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AssuredWorkloadsWorkloadEkmProvisioningResponseElRef {
        AssuredWorkloadsWorkloadEkmProvisioningResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AssuredWorkloadsWorkloadEkmProvisioningResponseElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ekm_provisioning_error_domain` after provisioning.\n"]
    pub fn ekm_provisioning_error_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ekm_provisioning_error_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `ekm_provisioning_error_mapping` after provisioning.\n"]
    pub fn ekm_provisioning_error_mapping(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ekm_provisioning_error_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `ekm_provisioning_state` after provisioning.\n"]
    pub fn ekm_provisioning_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ekm_provisioning_state", self.base))
    }
}

#[derive(Serialize)]
pub struct AssuredWorkloadsWorkloadResourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AssuredWorkloadsWorkloadResourcesEl {
    #[doc = "Set the field `resource_id`.\n"]
    pub fn set_resource_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.resource_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AssuredWorkloadsWorkloadResourcesEl {
    type O = terrars::BlockAssignable<AssuredWorkloadsWorkloadResourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAssuredWorkloadsWorkloadResourcesEl {}

impl BuildAssuredWorkloadsWorkloadResourcesEl {
    pub fn build(self) -> AssuredWorkloadsWorkloadResourcesEl {
        AssuredWorkloadsWorkloadResourcesEl {
            resource_id: core::default::Default::default(),
            resource_type: core::default::Default::default(),
        }
    }
}

pub struct AssuredWorkloadsWorkloadResourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AssuredWorkloadsWorkloadResourcesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AssuredWorkloadsWorkloadResourcesElRef {
        AssuredWorkloadsWorkloadResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AssuredWorkloadsWorkloadResourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }
}

#[derive(Serialize)]
pub struct AssuredWorkloadsWorkloadSaaEnrollmentResponseEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    setup_errors: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    setup_status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AssuredWorkloadsWorkloadSaaEnrollmentResponseEl {
    #[doc = "Set the field `setup_errors`.\n"]
    pub fn set_setup_errors(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.setup_errors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `setup_status`.\n"]
    pub fn set_setup_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.setup_status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AssuredWorkloadsWorkloadSaaEnrollmentResponseEl {
    type O = terrars::BlockAssignable<AssuredWorkloadsWorkloadSaaEnrollmentResponseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAssuredWorkloadsWorkloadSaaEnrollmentResponseEl {}

impl BuildAssuredWorkloadsWorkloadSaaEnrollmentResponseEl {
    pub fn build(self) -> AssuredWorkloadsWorkloadSaaEnrollmentResponseEl {
        AssuredWorkloadsWorkloadSaaEnrollmentResponseEl {
            setup_errors: core::default::Default::default(),
            setup_status: core::default::Default::default(),
        }
    }
}

pub struct AssuredWorkloadsWorkloadSaaEnrollmentResponseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AssuredWorkloadsWorkloadSaaEnrollmentResponseElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AssuredWorkloadsWorkloadSaaEnrollmentResponseElRef {
        AssuredWorkloadsWorkloadSaaEnrollmentResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AssuredWorkloadsWorkloadSaaEnrollmentResponseElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `setup_errors` after provisioning.\n"]
    pub fn setup_errors(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.setup_errors", self.base))
    }

    #[doc = "Get a reference to the value of field `setup_status` after provisioning.\n"]
    pub fn setup_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.setup_status", self.base))
    }
}

#[derive(Serialize)]
pub struct AssuredWorkloadsWorkloadKmsSettingsEl {
    next_rotation_time: terrars::PrimField<std::string::String>,
    rotation_period: terrars::PrimField<std::string::String>,
}

impl AssuredWorkloadsWorkloadKmsSettingsEl { }

impl terrars::ToListMappable for AssuredWorkloadsWorkloadKmsSettingsEl {
    type O = terrars::BlockAssignable<AssuredWorkloadsWorkloadKmsSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAssuredWorkloadsWorkloadKmsSettingsEl {
    #[doc =
        "Required. Input only. Immutable. The time at which the Key Management Service will automatically create a new version of the crypto key and mark it as the primary."]
    pub next_rotation_time: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. Input only. Immutable. will be advanced by this period when the Key Management Service automatically rotates a key. Must be at least 24 hours and at most 876,000 hours."]
    pub rotation_period: terrars::PrimField<std::string::String>,
}

impl BuildAssuredWorkloadsWorkloadKmsSettingsEl {
    pub fn build(self) -> AssuredWorkloadsWorkloadKmsSettingsEl {
        AssuredWorkloadsWorkloadKmsSettingsEl {
            next_rotation_time: self.next_rotation_time,
            rotation_period: self.rotation_period,
        }
    }
}

pub struct AssuredWorkloadsWorkloadKmsSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AssuredWorkloadsWorkloadKmsSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AssuredWorkloadsWorkloadKmsSettingsElRef {
        AssuredWorkloadsWorkloadKmsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AssuredWorkloadsWorkloadKmsSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `next_rotation_time` after provisioning.\nRequired. Input only. Immutable. The time at which the Key Management Service will automatically create a new version of the crypto key and mark it as the primary."]
    pub fn next_rotation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_rotation_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rotation_period` after provisioning.\nRequired. Input only. Immutable. will be advanced by this period when the Key Management Service automatically rotates a key. Must be at least 24 hours and at most 876,000 hours."]
    pub fn rotation_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rotation_period", self.base))
    }
}

#[derive(Serialize)]
pub struct AssuredWorkloadsWorkloadPartnerPermissionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assured_workloads_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_logs_viewer: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_access_approver: std::option::Option<terrars::PrimField<bool>>,
}

impl AssuredWorkloadsWorkloadPartnerPermissionsEl {
    #[doc = "Set the field `assured_workloads_monitoring`.\nOptional. Allow partner to view violation alerts."]
    pub fn set_assured_workloads_monitoring(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.assured_workloads_monitoring = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_logs_viewer`.\nAllow the partner to view inspectability logs and monitoring violations."]
    pub fn set_data_logs_viewer(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.data_logs_viewer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_access_approver`.\nOptional. Allow partner to view access approval logs."]
    pub fn set_service_access_approver(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.service_access_approver = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AssuredWorkloadsWorkloadPartnerPermissionsEl {
    type O = terrars::BlockAssignable<AssuredWorkloadsWorkloadPartnerPermissionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAssuredWorkloadsWorkloadPartnerPermissionsEl {}

impl BuildAssuredWorkloadsWorkloadPartnerPermissionsEl {
    pub fn build(self) -> AssuredWorkloadsWorkloadPartnerPermissionsEl {
        AssuredWorkloadsWorkloadPartnerPermissionsEl {
            assured_workloads_monitoring: core::default::Default::default(),
            data_logs_viewer: core::default::Default::default(),
            service_access_approver: core::default::Default::default(),
        }
    }
}

pub struct AssuredWorkloadsWorkloadPartnerPermissionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AssuredWorkloadsWorkloadPartnerPermissionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AssuredWorkloadsWorkloadPartnerPermissionsElRef {
        AssuredWorkloadsWorkloadPartnerPermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AssuredWorkloadsWorkloadPartnerPermissionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `assured_workloads_monitoring` after provisioning.\nOptional. Allow partner to view violation alerts."]
    pub fn assured_workloads_monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.assured_workloads_monitoring", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_logs_viewer` after provisioning.\nAllow the partner to view inspectability logs and monitoring violations."]
    pub fn data_logs_viewer(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_logs_viewer", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_access_approver` after provisioning.\nOptional. Allow partner to view access approval logs."]
    pub fn service_access_approver(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_access_approver", self.base))
    }
}

#[derive(Serialize)]
pub struct AssuredWorkloadsWorkloadResourceSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AssuredWorkloadsWorkloadResourceSettingsEl {
    #[doc =
        "Set the field `display_name`.\nUser-assigned resource display name. If not empty it will be used to create a resource with the specified name."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_id`.\nResource identifier. For a project this represents projectId. If the project is already taken, the workload creation will fail. For KeyRing, this represents the keyring_id. For a folder, don't set this value as folder_id is assigned by Google."]
    pub fn set_resource_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_type`.\nIndicates the type of resource. This field should be specified to correspond the id to the right project type (CONSUMER_PROJECT or ENCRYPTION_KEYS_PROJECT) Possible values: RESOURCE_TYPE_UNSPECIFIED, CONSUMER_PROJECT, ENCRYPTION_KEYS_PROJECT, KEYRING, CONSUMER_FOLDER"]
    pub fn set_resource_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AssuredWorkloadsWorkloadResourceSettingsEl {
    type O = terrars::BlockAssignable<AssuredWorkloadsWorkloadResourceSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAssuredWorkloadsWorkloadResourceSettingsEl {}

impl BuildAssuredWorkloadsWorkloadResourceSettingsEl {
    pub fn build(self) -> AssuredWorkloadsWorkloadResourceSettingsEl {
        AssuredWorkloadsWorkloadResourceSettingsEl {
            display_name: core::default::Default::default(),
            resource_id: core::default::Default::default(),
            resource_type: core::default::Default::default(),
        }
    }
}

pub struct AssuredWorkloadsWorkloadResourceSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AssuredWorkloadsWorkloadResourceSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AssuredWorkloadsWorkloadResourceSettingsElRef {
        AssuredWorkloadsWorkloadResourceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AssuredWorkloadsWorkloadResourceSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-assigned resource display name. If not empty it will be used to create a resource with the specified name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_id` after provisioning.\nResource identifier. For a project this represents projectId. If the project is already taken, the workload creation will fail. For KeyRing, this represents the keyring_id. For a folder, don't set this value as folder_id is assigned by Google."]
    pub fn resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_type` after provisioning.\nIndicates the type of resource. This field should be specified to correspond the id to the right project type (CONSUMER_PROJECT or ENCRYPTION_KEYS_PROJECT) Possible values: RESOURCE_TYPE_UNSPECIFIED, CONSUMER_PROJECT, ENCRYPTION_KEYS_PROJECT, KEYRING, CONSUMER_FOLDER"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }
}

#[derive(Serialize)]
pub struct AssuredWorkloadsWorkloadTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AssuredWorkloadsWorkloadTimeoutsEl {
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

impl terrars::ToListMappable for AssuredWorkloadsWorkloadTimeoutsEl {
    type O = terrars::BlockAssignable<AssuredWorkloadsWorkloadTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAssuredWorkloadsWorkloadTimeoutsEl {}

impl BuildAssuredWorkloadsWorkloadTimeoutsEl {
    pub fn build(self) -> AssuredWorkloadsWorkloadTimeoutsEl {
        AssuredWorkloadsWorkloadTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct AssuredWorkloadsWorkloadTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AssuredWorkloadsWorkloadTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AssuredWorkloadsWorkloadTimeoutsElRef {
        AssuredWorkloadsWorkloadTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AssuredWorkloadsWorkloadTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct AssuredWorkloadsWorkloadDynamic {
    kms_settings: std::option::Option<terrars::DynamicBlock<AssuredWorkloadsWorkloadKmsSettingsEl>>,
    partner_permissions: std::option::Option<terrars::DynamicBlock<AssuredWorkloadsWorkloadPartnerPermissionsEl>>,
    resource_settings: std::option::Option<terrars::DynamicBlock<AssuredWorkloadsWorkloadResourceSettingsEl>>,
}
