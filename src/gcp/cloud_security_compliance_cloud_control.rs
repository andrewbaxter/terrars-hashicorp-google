#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CloudSecurityComplianceCloudControlData_ {
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
    categories: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    cloud_control_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_category: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remediation_steps: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_cloud_providers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_spec: std::option::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_Rules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudSecurityComplianceCloudControl_Timeouts>,
    dynamic: CloudSecurityComplianceCloudControlDynamic,
}

struct CloudSecurityComplianceCloudControl_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudSecurityComplianceCloudControlData_>,
}

#[derive(Clone)]
pub struct CloudSecurityComplianceCloudControl(std::rc::Rc<CloudSecurityComplianceCloudControl_>);

impl CloudSecurityComplianceCloudControl {
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

    #[doc = "Set the field `categories`.\nThe categories of the cloud control."]
    pub fn set_categories(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().categories = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA description of the cloud control. The maximum length is 2000 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `display_name`.\nThe display name of the cloud control. The maximum length is 200\ncharacters."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `finding_category`.\nThe finding_category of the cloud control. The maximum length is 255\ncharacters."]
    pub fn set_finding_category(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().finding_category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `organization`.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn set_organization(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().organization = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parent`.\nThe parent resource in which to create the resource.\nMust be in one of the following formats:\n* 'projects/{{project}}'\n* 'organizations/{{organization}}'"]
    pub fn set_parent(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `remediation_steps`.\nThe remediation steps for the findings generated by the cloud control. The\nmaximum length is 400 characters."]
    pub fn set_remediation_steps(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().remediation_steps = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `severity`.\nPossible values:\nCRITICAL\nHIGH\nMEDIUM\nLOW"]
    pub fn set_severity(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().severity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `supported_cloud_providers`.\ncloud providers supported"]
    pub fn set_supported_cloud_providers(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().supported_cloud_providers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter_spec`.\n"]
    pub fn set_parameter_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameter_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameter_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudSecurityComplianceCloudControl_Rules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudSecurityComplianceCloudControl_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `categories` after provisioning.\nThe categories of the cloud control."]
    pub fn categories(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.categories", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_control_id` after provisioning.\nID of the CloudControl.\nThis is the last segment of the CloudControl resource name.\nFormat: '^a-zA-Z{0,61}[a-zA-Z0-9]$'."]
    pub fn cloud_control_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_control_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe last updated time of the cloud control.\nThe create_time is used because a new CC is created whenever we update an\nexisting CC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the cloud control. The maximum length is 2000 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the cloud control. The maximum length is 200\ncharacters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `finding_category` after provisioning.\nThe finding_category of the cloud control. The maximum length is 255\ncharacters."]
    pub fn finding_category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finding_category", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. Currently, only \"global\" is supported as a location."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `major_revision_id` after provisioning.\nMajor revision of the cloud control incremented in ascending order."]
    pub fn major_revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.major_revision_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the cloud control.\nFormat:\norganizations/{organization}/locations/{location}/cloudControls/{cloud_control_id}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn organization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe parent resource in which to create the resource.\nMust be in one of the following formats:\n* 'projects/{{project}}'\n* 'organizations/{{organization}}'"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `related_frameworks` after provisioning.\nThe Frameworks that include this CloudControl"]
    pub fn related_frameworks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.related_frameworks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remediation_steps` after provisioning.\nThe remediation steps for the findings generated by the cloud control. The\nmaximum length is 400 characters."]
    pub fn remediation_steps(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remediation_steps", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `severity` after provisioning.\nPossible values:\nCRITICAL\nHIGH\nMEDIUM\nLOW"]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_cloud_providers` after provisioning.\ncloud providers supported"]
    pub fn supported_cloud_providers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_cloud_providers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_enforcement_modes` after provisioning.\nThe supported enforcement mode of the cloud control. Default is DETECTIVE."]
    pub fn supported_enforcement_modes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_enforcement_modes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_target_resource_types` after provisioning.\ntarget resource types supported by the CloudControl."]
    pub fn supported_target_resource_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.supported_target_resource_types", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parameter_spec` after provisioning.\n"]
    pub fn parameter_spec(&self) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameter_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<CloudSecurityComplianceCloudControl_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudSecurityComplianceCloudControl_TimeoutsRef {
        CloudSecurityComplianceCloudControl_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for CloudSecurityComplianceCloudControl {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudSecurityComplianceCloudControl { }

impl terrars::ToListMappable for CloudSecurityComplianceCloudControl {
    type O = terrars::ListRef<CloudSecurityComplianceCloudControlRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudSecurityComplianceCloudControl_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_cloud_security_compliance_cloud_control".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudSecurityComplianceCloudControl {
    pub tf_id: std::string::String,
    #[doc =
        "ID of the CloudControl.\nThis is the last segment of the CloudControl resource name.\nFormat: '^a-zA-Z{0,61}[a-zA-Z0-9]$'."]
    pub cloud_control_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. Currently, only \"global\" is supported as a location."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildCloudSecurityComplianceCloudControl {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudSecurityComplianceCloudControl {
        let out = CloudSecurityComplianceCloudControl(std::rc::Rc::new(CloudSecurityComplianceCloudControl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudSecurityComplianceCloudControlData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                categories: core::default::Default::default(),
                cloud_control_id: self.cloud_control_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                finding_category: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                organization: core::default::Default::default(),
                parent: core::default::Default::default(),
                remediation_steps: core::default::Default::default(),
                severity: core::default::Default::default(),
                supported_cloud_providers: core::default::Default::default(),
                parameter_spec: core::default::Default::default(),
                rules: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudSecurityComplianceCloudControlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudSecurityComplianceCloudControlRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `categories` after provisioning.\nThe categories of the cloud control."]
    pub fn categories(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.categories", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_control_id` after provisioning.\nID of the CloudControl.\nThis is the last segment of the CloudControl resource name.\nFormat: '^a-zA-Z{0,61}[a-zA-Z0-9]$'."]
    pub fn cloud_control_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_control_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe last updated time of the cloud control.\nThe create_time is used because a new CC is created whenever we update an\nexisting CC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the cloud control. The maximum length is 2000 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the cloud control. The maximum length is 200\ncharacters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `finding_category` after provisioning.\nThe finding_category of the cloud control. The maximum length is 255\ncharacters."]
    pub fn finding_category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finding_category", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. Currently, only \"global\" is supported as a location."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `major_revision_id` after provisioning.\nMajor revision of the cloud control incremented in ascending order."]
    pub fn major_revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.major_revision_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the cloud control.\nFormat:\norganizations/{organization}/locations/{location}/cloudControls/{cloud_control_id}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn organization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe parent resource in which to create the resource.\nMust be in one of the following formats:\n* 'projects/{{project}}'\n* 'organizations/{{organization}}'"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `related_frameworks` after provisioning.\nThe Frameworks that include this CloudControl"]
    pub fn related_frameworks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.related_frameworks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remediation_steps` after provisioning.\nThe remediation steps for the findings generated by the cloud control. The\nmaximum length is 400 characters."]
    pub fn remediation_steps(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remediation_steps", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `severity` after provisioning.\nPossible values:\nCRITICAL\nHIGH\nMEDIUM\nLOW"]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_cloud_providers` after provisioning.\ncloud providers supported"]
    pub fn supported_cloud_providers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_cloud_providers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_enforcement_modes` after provisioning.\nThe supported enforcement mode of the cloud control. Default is DETECTIVE."]
    pub fn supported_enforcement_modes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_enforcement_modes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_target_resource_types` after provisioning.\ntarget resource types supported by the CloudControl."]
    pub fn supported_target_resource_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.supported_target_resource_types", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parameter_spec` after provisioning.\n"]
    pub fn parameter_spec(&self) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameter_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<CloudSecurityComplianceCloudControl_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudSecurityComplianceCloudControl_TimeoutsRef {
        CloudSecurityComplianceCloudControl_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlDynamic {
    parameter_spec: std::option::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec>>,
    rules: std::option::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_Rules>>,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue {
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue { }

impl terrars::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue {
    type O =
        terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue {
    #[doc = "The strings in the list."]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue {
        CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue { values: self.values }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValueRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\nThe strings in the list."]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValueDynamic {
    string_list_value: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue>>,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bool_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oneof_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_list_value: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue>>,
    dynamic: CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValueDynamic,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValue {
    #[doc = "Set the field `bool_value`.\nRepresents a boolean value."]
    pub fn set_bool_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.bool_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_value`.\nRepresents a double value."]
    pub fn set_number_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.number_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oneof_value`.\nSub-parameter values, in JSON format."]
    pub fn set_oneof_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.oneof_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_value`.\nRepresents a string value."]
    pub fn set_string_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_list_value`.\n"]
    pub fn set_string_list_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_list_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_list_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValue {
    type O =
        terrars
        ::BlockAssignable<
            CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValue {}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValue {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValue {
        CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValue {
            bool_value: core::default::Default::default(),
            number_value: core::default::Default::default(),
            oneof_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
            string_list_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValueRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bool_value` after provisioning.\nRepresents a boolean value."]
    pub fn bool_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bool_value", self.base))
    }

    #[doc = "Get a reference to the value of field `number_value` after provisioning.\nRepresents a double value."]
    pub fn number_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oneof_value` after provisioning.\nSub-parameter values, in JSON format."]
    pub fn oneof_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oneof_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_value` after provisioning.\nRepresents a string value."]
    pub fn string_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"]
    pub fn string_list_value(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_list_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValueDynamic {
    parameter_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValue,
        >,
    >,
    dynamic: CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValueDynamic,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue {
    #[doc = "Set the field `name`.\nThe name of the parameter."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter_value`.\n"]
    pub fn set_parameter_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameter_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue {
    type O =
        terrars
        ::BlockAssignable<
            CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue {}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue {
        CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue {
            name: core::default::Default::default(),
            parameter_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValueRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the parameter."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parameter_value` after provisioning.\n"]
    pub fn parameter_value(
        &self,
    ) -> terrars
    ::ListRef<
        CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue_ParameterValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameter_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValueDynamic {
    oneof_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue,
        >,
    >,
    string_list_value: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue>>,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bool_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oneof_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_list_value: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue>>,
    dynamic: CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValueDynamic,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue {
    #[doc = "Set the field `bool_value`.\nRepresents a boolean value."]
    pub fn set_bool_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.bool_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_value`.\nRepresents a double value."]
    pub fn set_number_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.number_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_value`.\nRepresents a string value."]
    pub fn set_string_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oneof_value`.\n"]
    pub fn set_oneof_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oneof_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oneof_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `string_list_value`.\n"]
    pub fn set_string_list_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_list_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_list_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue {
    type O =
        terrars
        ::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue {}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue {
        CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue {
            bool_value: core::default::Default::default(),
            number_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
            oneof_value: core::default::Default::default(),
            string_list_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValueRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bool_value` after provisioning.\nRepresents a boolean value."]
    pub fn bool_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bool_value", self.base))
    }

    #[doc = "Get a reference to the value of field `number_value` after provisioning.\nRepresents a double value."]
    pub fn number_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_value` after provisioning.\nRepresents a string value."]
    pub fn string_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }

    #[doc = "Get a reference to the value of field `oneof_value` after provisioning.\n"]
    pub fn oneof_value(
        &self,
    ) -> terrars
    ::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oneof_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"]
    pub fn string_list_value(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_list_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValueDynamic {
    parameter_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue>,
    >,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_value: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue>>,
    dynamic: CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValueDynamic,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue {
    #[doc = "Set the field `name`.\nThe name of the parameter."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter_value`.\n"]
    pub fn set_parameter_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameter_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue {
    type O = terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue {}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue {
        CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue {
            name: core::default::Default::default(),
            parameter_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValueRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the parameter."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parameter_value` after provisioning.\n"]
    pub fn parameter_value(
        &self,
    ) -> terrars
    ::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameter_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValueDynamic {
    oneof_value: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue>>,
    string_list_value: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue>>,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bool_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oneof_value: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_list_value: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue>>,
    dynamic: CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValueDynamic,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue {
    #[doc = "Set the field `bool_value`.\nRepresents a boolean value."]
    pub fn set_bool_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.bool_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_value`.\nRepresents a double value."]
    pub fn set_number_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.number_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_value`.\nRepresents a string value."]
    pub fn set_string_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oneof_value`.\n"]
    pub fn set_oneof_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oneof_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oneof_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `string_list_value`.\n"]
    pub fn set_string_list_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_list_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_list_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue {
    type O = terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue {}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue {
        CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue {
            bool_value: core::default::Default::default(),
            number_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
            oneof_value: core::default::Default::default(),
            string_list_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValueRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bool_value` after provisioning.\nRepresents a boolean value."]
    pub fn bool_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bool_value", self.base))
    }

    #[doc = "Get a reference to the value of field `number_value` after provisioning.\nRepresents a double value."]
    pub fn number_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_value` after provisioning.\nRepresents a string value."]
    pub fn string_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }

    #[doc = "Get a reference to the value of field `oneof_value` after provisioning.\n"]
    pub fn oneof_value(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oneof_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"]
    pub fn string_list_value(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_list_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValueDynamic {
    oneof_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue,
        >,
    >,
    string_list_value: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue>>,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bool_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oneof_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_list_value: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue>>,
    dynamic: CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValueDynamic,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValue {
    #[doc = "Set the field `bool_value`.\nRepresents a boolean value."]
    pub fn set_bool_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.bool_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_value`.\nRepresents a double value."]
    pub fn set_number_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.number_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_value`.\nRepresents a string value."]
    pub fn set_string_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oneof_value`.\n"]
    pub fn set_oneof_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oneof_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oneof_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `string_list_value`.\n"]
    pub fn set_string_list_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_list_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_list_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValue {
    type O =
        terrars
        ::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValue {}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValue {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValue {
        CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValue {
            bool_value: core::default::Default::default(),
            number_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
            oneof_value: core::default::Default::default(),
            string_list_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValueRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bool_value` after provisioning.\nRepresents a boolean value."]
    pub fn bool_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bool_value", self.base))
    }

    #[doc = "Get a reference to the value of field `number_value` after provisioning.\nRepresents a double value."]
    pub fn number_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_value` after provisioning.\nRepresents a string value."]
    pub fn string_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }

    #[doc = "Get a reference to the value of field `oneof_value` after provisioning.\n"]
    pub fn oneof_value(
        &self,
    ) -> terrars
    ::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oneof_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"]
    pub fn string_list_value(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_list_value", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_AttributeSubstitutionRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_AttributeSubstitutionRule {
    #[doc =
        "Set the field `attribute`.\nFully qualified proto attribute path (in dot notation).\nExample: rules[0].cel_expression.resource_types_values"]
    pub fn set_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_AttributeSubstitutionRule {
    type O =
        terrars
        ::BlockAssignable<
            CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_AttributeSubstitutionRule,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_AttributeSubstitutionRule {}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_AttributeSubstitutionRule {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_AttributeSubstitutionRule {
        CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_AttributeSubstitutionRule {
            attribute: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_AttributeSubstitutionRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_AttributeSubstitutionRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_AttributeSubstitutionRuleRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_AttributeSubstitutionRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_AttributeSubstitutionRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attribute` after provisioning.\nFully qualified proto attribute path (in dot notation).\nExample: rules[0].cel_expression.resource_types_values"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_PlaceholderSubstitutionRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_PlaceholderSubstitutionRule {
    #[doc = "Set the field `attribute`.\nFully qualified proto attribute path (e.g., dot notation)"]
    pub fn set_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_PlaceholderSubstitutionRule {
    type O =
        terrars
        ::BlockAssignable<
            CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_PlaceholderSubstitutionRule,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_PlaceholderSubstitutionRule {}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_PlaceholderSubstitutionRule {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_PlaceholderSubstitutionRule {
        CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_PlaceholderSubstitutionRule {
            attribute: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_PlaceholderSubstitutionRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_PlaceholderSubstitutionRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_PlaceholderSubstitutionRuleRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_PlaceholderSubstitutionRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_PlaceholderSubstitutionRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attribute` after provisioning.\nFully qualified proto attribute path (e.g., dot notation)"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRulesDynamic {
    attribute_substitution_rule: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_AttributeSubstitutionRule>,
    >,
    placeholder_substitution_rule: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_PlaceholderSubstitutionRule,
        >,
    >,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_substitution_rule: std
    ::option
    ::Option<
        std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_AttributeSubstitutionRule>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placeholder_substitution_rule: std
    ::option
    ::Option<
        std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_PlaceholderSubstitutionRule>,
    >,
    dynamic: CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRulesDynamic,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules {
    #[doc = "Set the field `attribute_substitution_rule`.\n"]
    pub fn set_attribute_substitution_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_AttributeSubstitutionRule,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.attribute_substitution_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.attribute_substitution_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placeholder_substitution_rule`.\n"]
    pub fn set_placeholder_substitution_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_PlaceholderSubstitutionRule,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.placeholder_substitution_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.placeholder_substitution_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules {
    type O = terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules {}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules {
        CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules {
            attribute_substitution_rule: core::default::Default::default(),
            placeholder_substitution_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRulesRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_substitution_rule` after provisioning.\n"]
    pub fn attribute_substitution_rule(
        &self,
    ) -> terrars
    ::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_AttributeSubstitutionRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attribute_substitution_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `placeholder_substitution_rule` after provisioning.\n"]
    pub fn placeholder_substitution_rule(
        &self,
    ) -> terrars
    ::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules_PlaceholderSubstitutionRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placeholder_substitution_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_ValuesDynamic {
    oneof_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue,
        >,
    >,
    string_list_value: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue>>,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_Values {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bool_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oneof_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_list_value: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue>>,
    dynamic: CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_ValuesDynamic,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_Values {
    #[doc = "Set the field `bool_value`.\nRepresents a boolean value."]
    pub fn set_bool_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.bool_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_value`.\nRepresents a double value."]
    pub fn set_number_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.number_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_value`.\nRepresents a string value."]
    pub fn set_string_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oneof_value`.\n"]
    pub fn set_oneof_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oneof_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oneof_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `string_list_value`.\n"]
    pub fn set_string_list_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_list_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_list_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_Values {
    type O =
        terrars
        ::BlockAssignable<
            CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_Values,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_Values {}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_Values {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_Values {
        CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_Values {
            bool_value: core::default::Default::default(),
            number_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
            oneof_value: core::default::Default::default(),
            string_list_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_ValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_ValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_ValuesRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_ValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_ValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bool_value` after provisioning.\nRepresents a boolean value."]
    pub fn bool_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bool_value", self.base))
    }

    #[doc = "Get a reference to the value of field `number_value` after provisioning.\nRepresents a double value."]
    pub fn number_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_value` after provisioning.\nRepresents a string value."]
    pub fn string_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }

    #[doc = "Get a reference to the value of field `oneof_value` after provisioning.\n"]
    pub fn oneof_value(
        &self,
    ) -> terrars
    ::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue_ParameterValue_OneofValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oneof_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"]
    pub fn string_list_value(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_list_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValuesDynamic {
    values: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_Values,
        >,
    >,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_Values,
        >,
    >,
    dynamic: CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValuesDynamic,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues {
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_Values,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.values = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues {
    type O =
        terrars
        ::BlockAssignable<
            CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues {}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues {
        CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues {
            values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValuesRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(
        &self,
    ) -> terrars
    ::ListRef<
        CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues_ValuesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRange {
    max: terrars::PrimField<std::string::String>,
    min: terrars::PrimField<std::string::String>,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRange { }

impl terrars::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRange {
    type O = terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRange {
    #[doc = "Maximum allowed value for the numeric parameter (inclusive)."]
    pub max: terrars::PrimField<std::string::String>,
    #[doc = "Minimum allowed value for the numeric parameter (inclusive)."]
    pub min: terrars::PrimField<std::string::String>,
}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRange {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRange {
        CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRange {
            max: self.max,
            min: self.min,
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRangeRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max` after provisioning.\nMaximum allowed value for the numeric parameter (inclusive)."]
    pub fn max(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min` after provisioning.\nMinimum allowed value for the numeric parameter (inclusive)."]
    pub fn min(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPattern {
    pattern: terrars::PrimField<std::string::String>,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPattern { }

impl terrars::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPattern {
    type O = terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPattern>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPattern {
    #[doc = "Regex Pattern to match the value(s) of parameter."]
    pub pattern: terrars::PrimField<std::string::String>,
}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPattern {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPattern {
        CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPattern { pattern: self.pattern }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPatternRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPatternRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPatternRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPatternRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPatternRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pattern` after provisioning.\nRegex Pattern to match the value(s) of parameter."]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_ValidationDynamic {
    allowed_values: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues,
        >,
    >,
    int_range: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRange>>,
    regexp_pattern: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPattern>>,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_values: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    int_range: std::option::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRange>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regexp_pattern: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPattern>>,
    dynamic: CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_ValidationDynamic,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation {
    #[doc = "Set the field `allowed_values`.\n"]
    pub fn set_allowed_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValues,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowed_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_values = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `int_range`.\n"]
    pub fn set_int_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRange>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.int_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.int_range = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regexp_pattern`.\n"]
    pub fn set_regexp_pattern(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPattern>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regexp_pattern = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regexp_pattern = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation {
    type O =
        terrars
        ::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation {}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation {
        CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation {
            allowed_values: core::default::Default::default(),
            int_range: core::default::Default::default(),
            regexp_pattern: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_ValidationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_ValidationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_ValidationRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_ValidationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_ValidationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_values` after provisioning.\n"]
    pub fn allowed_values(
        &self,
    ) -> terrars
    ::ListRef<
        CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation_AllowedValuesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_values", self.base))
    }

    #[doc = "Get a reference to the value of field `int_range` after provisioning.\n"]
    pub fn int_range(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.int_range", self.base))
    }

    #[doc = "Get a reference to the value of field `regexp_pattern` after provisioning.\n"]
    pub fn regexp_pattern(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPatternRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regexp_pattern", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParametersDynamic {
    default_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValue>,
    >,
    substitution_rules: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules>>,
    validation: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation>,
    >,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    is_required: terrars::PrimField<bool>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sub_parameters: std::option::Option<terrars::PrimField<std::string::String>>,
    value_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_value: std
    ::option
    ::Option<
        std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValue>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    substitution_rules: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation>>,
    dynamic: CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParametersDynamic,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters {
    #[doc = "Set the field `description`.\nThe description of the parameter. The maximum length is 2000 characters."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nThe display name of the parameter. The maximum length is 200 characters."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sub_parameters`.\nThe parameter spec of the cloud control, in JSON format."]
    pub fn set_sub_parameters(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sub_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `substitution_rules`.\n"]
    pub fn set_substitution_rules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.substitution_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.substitution_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_Validation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.validation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.validation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters {
    type O = terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters {
    #[doc = "if the parameter is required"]
    pub is_required: terrars::PrimField<bool>,
    #[doc = "The name of the parameter."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Parameter value type.\nPossible values:\nSTRING\nBOOLEAN\nSTRINGLIST\nNUMBER\nONEOF"]
    pub value_type: terrars::PrimField<std::string::String>,
}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters {
        CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters {
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            is_required: self.is_required,
            name: self.name,
            sub_parameters: core::default::Default::default(),
            value_type: self.value_type,
            default_value: core::default::Default::default(),
            substitution_rules: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParametersRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the parameter. The maximum length is 2000 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the parameter. The maximum length is 200 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `is_required` after provisioning.\nif the parameter is required"]
    pub fn is_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_required", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the parameter."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sub_parameters` after provisioning.\nThe parameter spec of the cloud control, in JSON format."]
    pub fn sub_parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sub_parameters", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value_type` after provisioning.\nParameter value type.\nPossible values:\nSTRING\nBOOLEAN\nSTRINGLIST\nNUMBER\nONEOF"]
    pub fn value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(
        &self,
    ) -> terrars
    ::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_DefaultValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_value", self.base))
    }

    #[doc = "Get a reference to the value of field `substitution_rules` after provisioning.\n"]
    pub fn substitution_rules(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.substitution_rules", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(
        &self,
    ) -> terrars
    ::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters_ValidationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_ValuesDynamic {
    oneof_value: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue>>,
    string_list_value: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue>>,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_Values {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bool_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oneof_value: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_list_value: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue>>,
    dynamic: CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_ValuesDynamic,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_Values {
    #[doc = "Set the field `bool_value`.\nRepresents a boolean value."]
    pub fn set_bool_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.bool_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_value`.\nRepresents a double value."]
    pub fn set_number_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.number_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_value`.\nRepresents a string value."]
    pub fn set_string_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oneof_value`.\n"]
    pub fn set_oneof_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValue>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oneof_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oneof_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `string_list_value`.\n"]
    pub fn set_string_list_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_list_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_list_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_Values {
    type O =
        terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_Values>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_Values {}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_Values {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_Values {
        CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_Values {
            bool_value: core::default::Default::default(),
            number_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
            oneof_value: core::default::Default::default(),
            string_list_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_ValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_ValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_ValuesRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_ValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_ValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bool_value` after provisioning.\nRepresents a boolean value."]
    pub fn bool_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bool_value", self.base))
    }

    #[doc = "Get a reference to the value of field `number_value` after provisioning.\nRepresents a double value."]
    pub fn number_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_value` after provisioning.\nRepresents a string value."]
    pub fn string_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }

    #[doc = "Get a reference to the value of field `oneof_value` after provisioning.\n"]
    pub fn oneof_value(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_OneofValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oneof_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"]
    pub fn string_list_value(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue_StringListValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_list_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValuesDynamic {
    values: std
    ::option
    ::Option<
        terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_Values>,
    >,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_Values>>,
    dynamic: CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValuesDynamic,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues {
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_Values,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.values = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues {
    type O = terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues {}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues {
        CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues {
            values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValuesRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues_ValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControl_ParameterSpec_ValidationDynamic {
    allowed_values: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues>>,
    int_range: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRange>>,
    regexp_pattern: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPattern>>,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_Validation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_values: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    int_range: std::option::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRange>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regexp_pattern: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPattern>>,
    dynamic: CloudSecurityComplianceCloudControl_ParameterSpec_ValidationDynamic,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_Validation {
    #[doc = "Set the field `allowed_values`.\n"]
    pub fn set_allowed_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValues>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowed_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_values = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `int_range`.\n"]
    pub fn set_int_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRange>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.int_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.int_range = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regexp_pattern`.\n"]
    pub fn set_regexp_pattern(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPattern>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regexp_pattern = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regexp_pattern = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_Validation {
    type O = terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_Validation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_Validation {}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_Validation {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_ParameterSpec_Validation {
        CloudSecurityComplianceCloudControl_ParameterSpec_Validation {
            allowed_values: core::default::Default::default(),
            int_range: core::default::Default::default(),
            regexp_pattern: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_ValidationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_ValidationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_ValidationRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_ValidationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_ValidationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_values` after provisioning.\n"]
    pub fn allowed_values(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_AllowedValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_values", self.base))
    }

    #[doc = "Get a reference to the value of field `int_range` after provisioning.\n"]
    pub fn int_range(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_IntRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.int_range", self.base))
    }

    #[doc = "Get a reference to the value of field `regexp_pattern` after provisioning.\n"]
    pub fn regexp_pattern(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_Validation_RegexpPatternRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regexp_pattern", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControl_ParameterSpec_SubParametersDynamic {
    default_value: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue>>,
    sub_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters>>,
    substitution_rules: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules>>,
    validation: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_Validation>>,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    is_required: terrars::PrimField<bool>,
    name: terrars::PrimField<std::string::String>,
    value_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_value: std::option::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sub_parameters: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    substitution_rules: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation: std::option::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_Validation>>,
    dynamic: CloudSecurityComplianceCloudControl_ParameterSpec_SubParametersDynamic,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters {
    #[doc = "Set the field `description`.\nThe description of the parameter. The maximum length is 2000 characters."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nThe display name of the parameter. The maximum length is 200 characters."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sub_parameters`.\n"]
    pub fn set_sub_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParameters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sub_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sub_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `substitution_rules`.\n"]
    pub fn set_substitution_rules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.substitution_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.substitution_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_Validation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.validation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.validation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters {
    type O = terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec_SubParameters {
    #[doc = "if the parameter is required"]
    pub is_required: terrars::PrimField<bool>,
    #[doc = "The name of the parameter."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Parameter value type.\nPossible values:\nSTRING\nBOOLEAN\nSTRINGLIST\nNUMBER\nONEOF"]
    pub value_type: terrars::PrimField<std::string::String>,
}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec_SubParameters {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters {
        CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters {
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            is_required: self.is_required,
            name: self.name,
            value_type: self.value_type,
            default_value: core::default::Default::default(),
            sub_parameters: core::default::Default::default(),
            substitution_rules: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec_SubParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_ParameterSpec_SubParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpec_SubParametersRef {
        CloudSecurityComplianceCloudControl_ParameterSpec_SubParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpec_SubParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the parameter. The maximum length is 2000 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the parameter. The maximum length is 200 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `is_required` after provisioning.\nif the parameter is required"]
    pub fn is_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_required", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the parameter."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value_type` after provisioning.\nParameter value type.\nPossible values:\nSTRING\nBOOLEAN\nSTRINGLIST\nNUMBER\nONEOF"]
    pub fn value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_value", self.base))
    }

    #[doc = "Get a reference to the value of field `sub_parameters` after provisioning.\n"]
    pub fn sub_parameters(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters_SubParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sub_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `substitution_rules` after provisioning.\n"]
    pub fn substitution_rules(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.substitution_rules", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(&self) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_ValidationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControl_ParameterSpecDynamic {
    default_value: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue>>,
    sub_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters>>,
    substitution_rules: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules>>,
    validation: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_ParameterSpec_Validation>>,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    is_required: terrars::PrimField<bool>,
    name: terrars::PrimField<std::string::String>,
    value_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_value: std::option::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sub_parameters: std::option::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    substitution_rules: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation: std::option::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_ParameterSpec_Validation>>,
    dynamic: CloudSecurityComplianceCloudControl_ParameterSpecDynamic,
}

impl CloudSecurityComplianceCloudControl_ParameterSpec {
    #[doc = "Set the field `description`.\nThe description of the parameter. The maximum length is 2000 characters."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nThe display name of the parameter. The maximum length is 200 characters."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValue>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sub_parameters`.\n"]
    pub fn set_sub_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_SubParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sub_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sub_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `substitution_rules`.\n"]
    pub fn set_substitution_rules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRules>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.substitution_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.substitution_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec_Validation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.validation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.validation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudSecurityComplianceCloudControl_ParameterSpec {
    type O = terrars::BlockAssignable<CloudSecurityComplianceCloudControl_ParameterSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_ParameterSpec {
    #[doc = "if the parameter is required"]
    pub is_required: terrars::PrimField<bool>,
    #[doc = "The name of the parameter."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Parameter value type.\nPossible values:\nSTRING\nBOOLEAN\nSTRINGLIST\nNUMBER\nONEOF"]
    pub value_type: terrars::PrimField<std::string::String>,
}

impl BuildCloudSecurityComplianceCloudControl_ParameterSpec {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_ParameterSpec {
        CloudSecurityComplianceCloudControl_ParameterSpec {
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            is_required: self.is_required,
            name: self.name,
            value_type: self.value_type,
            default_value: core::default::Default::default(),
            sub_parameters: core::default::Default::default(),
            substitution_rules: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_ParameterSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_ParameterSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_ParameterSpecRef {
        CloudSecurityComplianceCloudControl_ParameterSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_ParameterSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the parameter. The maximum length is 2000 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the parameter. The maximum length is 200 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `is_required` after provisioning.\nif the parameter is required"]
    pub fn is_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_required", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the parameter."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value_type` after provisioning.\nParameter value type.\nPossible values:\nSTRING\nBOOLEAN\nSTRINGLIST\nNUMBER\nONEOF"]
    pub fn value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_DefaultValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_value", self.base))
    }

    #[doc = "Get a reference to the value of field `sub_parameters` after provisioning.\n"]
    pub fn sub_parameters(&self) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_SubParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sub_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `substitution_rules` after provisioning.\n"]
    pub fn substitution_rules(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_SubstitutionRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.substitution_rules", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(&self) -> terrars::ListRef<CloudSecurityComplianceCloudControl_ParameterSpec_ValidationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_Rules_CelExpression_ResourceTypesValues {
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl CloudSecurityComplianceCloudControl_Rules_CelExpression_ResourceTypesValues { }

impl terrars::ToListMappable for CloudSecurityComplianceCloudControl_Rules_CelExpression_ResourceTypesValues {
    type O = terrars::BlockAssignable<CloudSecurityComplianceCloudControl_Rules_CelExpression_ResourceTypesValues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_Rules_CelExpression_ResourceTypesValues {
    #[doc = "The strings in the list."]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudSecurityComplianceCloudControl_Rules_CelExpression_ResourceTypesValues {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_Rules_CelExpression_ResourceTypesValues {
        CloudSecurityComplianceCloudControl_Rules_CelExpression_ResourceTypesValues { values: self.values }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_Rules_CelExpression_ResourceTypesValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_Rules_CelExpression_ResourceTypesValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_Rules_CelExpression_ResourceTypesValuesRef {
        CloudSecurityComplianceCloudControl_Rules_CelExpression_ResourceTypesValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_Rules_CelExpression_ResourceTypesValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\nThe strings in the list."]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControl_Rules_CelExpressionDynamic {
    resource_types_values: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_Rules_CelExpression_ResourceTypesValues>>,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_Rules_CelExpression {
    expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_types_values: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_Rules_CelExpression_ResourceTypesValues>>,
    dynamic: CloudSecurityComplianceCloudControl_Rules_CelExpressionDynamic,
}

impl CloudSecurityComplianceCloudControl_Rules_CelExpression {
    #[doc = "Set the field `resource_types_values`.\n"]
    pub fn set_resource_types_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudSecurityComplianceCloudControl_Rules_CelExpression_ResourceTypesValues>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_types_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_types_values = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudSecurityComplianceCloudControl_Rules_CelExpression {
    type O = terrars::BlockAssignable<CloudSecurityComplianceCloudControl_Rules_CelExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_Rules_CelExpression {
    #[doc = "Logic expression in CEL language.\nThe max length of the condition is 1000 characters."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildCloudSecurityComplianceCloudControl_Rules_CelExpression {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_Rules_CelExpression {
        CloudSecurityComplianceCloudControl_Rules_CelExpression {
            expression: self.expression,
            resource_types_values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_Rules_CelExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_Rules_CelExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_Rules_CelExpressionRef {
        CloudSecurityComplianceCloudControl_Rules_CelExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_Rules_CelExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nLogic expression in CEL language.\nThe max length of the condition is 1000 characters."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_types_values` after provisioning.\n"]
    pub fn resource_types_values(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceCloudControl_Rules_CelExpression_ResourceTypesValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_types_values", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControl_RulesDynamic {
    cel_expression: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceCloudControl_Rules_CelExpression>>,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    rule_action_types: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cel_expression: std::option::Option<std::vec::Vec<CloudSecurityComplianceCloudControl_Rules_CelExpression>>,
    dynamic: CloudSecurityComplianceCloudControl_RulesDynamic,
}

impl CloudSecurityComplianceCloudControl_Rules {
    #[doc = "Set the field `description`.\nDescription of the Rule. The maximum length is 2000 characters."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cel_expression`.\n"]
    pub fn set_cel_expression(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudSecurityComplianceCloudControl_Rules_CelExpression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cel_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cel_expression = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudSecurityComplianceCloudControl_Rules {
    type O = terrars::BlockAssignable<CloudSecurityComplianceCloudControl_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_Rules {
    #[doc = "The functionality enabled by the Rule."]
    pub rule_action_types: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudSecurityComplianceCloudControl_Rules {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_Rules {
        CloudSecurityComplianceCloudControl_Rules {
            description: core::default::Default::default(),
            rule_action_types: self.rule_action_types,
            cel_expression: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudSecurityComplianceCloudControl_RulesRef {
        CloudSecurityComplianceCloudControl_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the Rule. The maximum length is 2000 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rule_action_types` after provisioning.\nThe functionality enabled by the Rule."]
    pub fn rule_action_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_action_types", self.base))
    }

    #[doc = "Get a reference to the value of field `cel_expression` after provisioning.\n"]
    pub fn cel_expression(&self) -> terrars::ListRef<CloudSecurityComplianceCloudControl_Rules_CelExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cel_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControl_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudSecurityComplianceCloudControl_Timeouts {
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

impl terrars::ToListMappable for CloudSecurityComplianceCloudControl_Timeouts {
    type O = terrars::BlockAssignable<CloudSecurityComplianceCloudControl_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceCloudControl_Timeouts {}

impl BuildCloudSecurityComplianceCloudControl_Timeouts {
    pub fn build(self) -> CloudSecurityComplianceCloudControl_Timeouts {
        CloudSecurityComplianceCloudControl_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceCloudControl_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceCloudControl_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceCloudControl_TimeoutsRef {
        CloudSecurityComplianceCloudControl_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceCloudControl_TimeoutsRef {
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
