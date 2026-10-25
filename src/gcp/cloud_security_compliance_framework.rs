#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CloudSecurityComplianceFrameworkData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    framework_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_control_details: std::option::Option<std::vec::Vec<CloudSecurityComplianceFramework_CloudControlDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudSecurityComplianceFramework_Timeouts>,
    dynamic: CloudSecurityComplianceFrameworkDynamic,
}

struct CloudSecurityComplianceFramework_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudSecurityComplianceFrameworkData_>,
}

#[derive(Clone)]
pub struct CloudSecurityComplianceFramework(std::rc::Rc<CloudSecurityComplianceFramework_>);

impl CloudSecurityComplianceFramework {
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

    #[doc = "Set the field `description`.\nThe description of the framework. The maximum length is 2000 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nDisplay name of the framework. The maximum length is 200 characters."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `cloud_control_details`.\n"]
    pub fn set_cloud_control_details(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudSecurityComplianceFramework_CloudControlDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_control_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_control_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudSecurityComplianceFramework_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\nThe category of the framework."]
    pub fn category(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.category", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the framework. The maximum length is 2000 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the framework. The maximum length is 200 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `framework_id` after provisioning.\nID of the framework.\nThis is not the full name of the framework.\nThis is the last part of the full name of the framework."]
    pub fn framework_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framework_id", self.extract_ref()))
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
        "Get a reference to the value of field `major_revision_id` after provisioning.\nMajor revision of the framework incremented in ascending order."]
    pub fn major_revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.major_revision_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the framework.\nFormat:\n{parent}/locations/{location}/frameworks/{framework_id}"]
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
        "Get a reference to the value of field `supported_cloud_providers` after provisioning.\ncloud providers supported"]
    pub fn supported_cloud_providers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_cloud_providers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_enforcement_modes` after provisioning.\nThe supported enforcement modes of the framework."]
    pub fn supported_enforcement_modes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.supported_enforcement_modes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_target_resource_types` after provisioning.\ntarget resource types supported by the Framework."]
    pub fn supported_target_resource_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.supported_target_resource_types", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the framework. The default is TYPE_CUSTOM.\nPossible values:\nBUILT_IN\nCUSTOM"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudSecurityComplianceFramework_TimeoutsRef {
        CloudSecurityComplianceFramework_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for CloudSecurityComplianceFramework {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudSecurityComplianceFramework { }

impl terrars::ToListMappable for CloudSecurityComplianceFramework {
    type O = terrars::ListRef<CloudSecurityComplianceFrameworkRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudSecurityComplianceFramework_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_cloud_security_compliance_framework".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudSecurityComplianceFramework {
    pub tf_id: std::string::String,
    #[doc =
        "ID of the framework.\nThis is not the full name of the framework.\nThis is the last part of the full name of the framework."]
    pub framework_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildCloudSecurityComplianceFramework {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudSecurityComplianceFramework {
        let out = CloudSecurityComplianceFramework(std::rc::Rc::new(CloudSecurityComplianceFramework_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudSecurityComplianceFrameworkData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                framework_id: self.framework_id,
                id: core::default::Default::default(),
                location: self.location,
                organization: core::default::Default::default(),
                parent: core::default::Default::default(),
                cloud_control_details: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudSecurityComplianceFrameworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceFrameworkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudSecurityComplianceFrameworkRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\nThe category of the framework."]
    pub fn category(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.category", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the framework. The maximum length is 2000 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the framework. The maximum length is 200 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `framework_id` after provisioning.\nID of the framework.\nThis is not the full name of the framework.\nThis is the last part of the full name of the framework."]
    pub fn framework_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framework_id", self.extract_ref()))
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
        "Get a reference to the value of field `major_revision_id` after provisioning.\nMajor revision of the framework incremented in ascending order."]
    pub fn major_revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.major_revision_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the framework.\nFormat:\n{parent}/locations/{location}/frameworks/{framework_id}"]
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
        "Get a reference to the value of field `supported_cloud_providers` after provisioning.\ncloud providers supported"]
    pub fn supported_cloud_providers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_cloud_providers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_enforcement_modes` after provisioning.\nThe supported enforcement modes of the framework."]
    pub fn supported_enforcement_modes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.supported_enforcement_modes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_target_resource_types` after provisioning.\ntarget resource types supported by the Framework."]
    pub fn supported_target_resource_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.supported_target_resource_types", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the framework. The default is TYPE_CUSTOM.\nPossible values:\nBUILT_IN\nCUSTOM"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudSecurityComplianceFramework_TimeoutsRef {
        CloudSecurityComplianceFramework_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkDynamic {
    cloud_control_details: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceFramework_CloudControlDetails>>,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValue {
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValue { }

impl terrars
::ToListMappable for CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValue {
    type O =
        terrars
        ::BlockAssignable<
            CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValue {
    #[doc = "The strings in the list."]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValue {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValue {
        CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValue {
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValueRef {
        CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValueRef {
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
struct CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValueDynamic {
    string_list_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValue>,
    >,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bool_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_list_value: std
    ::option
    ::Option<
        std::vec::Vec<CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValue>,
    >,
    dynamic: CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValueDynamic,
}

impl CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue {
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
                            CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValue,
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
::ToListMappable for CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue {
    type O =
        terrars
        ::BlockAssignable<
            CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue {}

impl BuildCloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue {
        CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue {
            bool_value: core::default::Default::default(),
            number_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
            string_list_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValueRef {
        CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValueRef {
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

    #[doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"]
    pub fn string_list_value(
        &self,
    ) -> terrars
    ::ListRef<CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_list_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValueDynamic {
    parameter_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue,
        >,
    >,
    dynamic: CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValueDynamic,
}

impl CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue {
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
                            CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue,
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
::ToListMappable for CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue {
    type O =
        terrars
        ::BlockAssignable<CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue {}

impl BuildCloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue {
    pub fn build(self) -> CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue {
        CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue {
            name: core::default::Default::default(),
            parameter_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValueRef {
        CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValueRef {
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
        CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameter_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValueDynamic {
    oneof_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue>,
    >,
    string_list_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValue>,
    >,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue {
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
        std::vec::Vec<CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_list_value: std
    ::option
    ::Option<
        std::vec::Vec<CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValue>,
    >,
    dynamic: CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValueDynamic,
}

impl CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue {
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
                            CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValue,
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
                            CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValue,
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

impl terrars::ToListMappable for CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue {
    type O =
        terrars::BlockAssignable<CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue {}

impl BuildCloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue {
    pub fn build(self) -> CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue {
        CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue {
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
pub struct CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValueRef {
        CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValueRef {
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
    ::ListRef<CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_OneofValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oneof_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"]
    pub fn string_list_value(
        &self,
    ) -> terrars
    ::ListRef<CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue_StringListValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_list_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceFramework_CloudControlDetails_ParametersDynamic {
    parameter_value: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue>>,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFramework_CloudControlDetails_Parameters {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_value: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue>>,
    dynamic: CloudSecurityComplianceFramework_CloudControlDetails_ParametersDynamic,
}

impl CloudSecurityComplianceFramework_CloudControlDetails_Parameters {
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
                            CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValue,
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

impl terrars::ToListMappable for CloudSecurityComplianceFramework_CloudControlDetails_Parameters {
    type O = terrars::BlockAssignable<CloudSecurityComplianceFramework_CloudControlDetails_Parameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFramework_CloudControlDetails_Parameters {
    #[doc = "The name of the parameter."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudSecurityComplianceFramework_CloudControlDetails_Parameters {
    pub fn build(self) -> CloudSecurityComplianceFramework_CloudControlDetails_Parameters {
        CloudSecurityComplianceFramework_CloudControlDetails_Parameters {
            name: self.name,
            parameter_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFramework_CloudControlDetails_ParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceFramework_CloudControlDetails_ParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFramework_CloudControlDetails_ParametersRef {
        CloudSecurityComplianceFramework_CloudControlDetails_ParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFramework_CloudControlDetails_ParametersRef {
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
    ) -> terrars::ListRef<CloudSecurityComplianceFramework_CloudControlDetails_Parameters_ParameterValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameter_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceFramework_CloudControlDetailsDynamic {
    parameters: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceFramework_CloudControlDetails_Parameters>>,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFramework_CloudControlDetails {
    major_revision_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<std::vec::Vec<CloudSecurityComplianceFramework_CloudControlDetails_Parameters>>,
    dynamic: CloudSecurityComplianceFramework_CloudControlDetailsDynamic,
}

impl CloudSecurityComplianceFramework_CloudControlDetails {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudSecurityComplianceFramework_CloudControlDetails_Parameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudSecurityComplianceFramework_CloudControlDetails {
    type O = terrars::BlockAssignable<CloudSecurityComplianceFramework_CloudControlDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFramework_CloudControlDetails {
    #[doc = "Major revision of cloudcontrol"]
    pub major_revision_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the CloudControl in the format:\n\"{parent}/locations/{location}/cloudControls/{cloud-control}\""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudSecurityComplianceFramework_CloudControlDetails {
    pub fn build(self) -> CloudSecurityComplianceFramework_CloudControlDetails {
        CloudSecurityComplianceFramework_CloudControlDetails {
            major_revision_id: self.major_revision_id,
            name: self.name,
            parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFramework_CloudControlDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceFramework_CloudControlDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFramework_CloudControlDetailsRef {
        CloudSecurityComplianceFramework_CloudControlDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFramework_CloudControlDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `major_revision_id` after provisioning.\nMajor revision of cloudcontrol"]
    pub fn major_revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.major_revision_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the CloudControl in the format:\n\"{parent}/locations/{location}/cloudControls/{cloud-control}\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<CloudSecurityComplianceFramework_CloudControlDetails_ParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFramework_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudSecurityComplianceFramework_Timeouts {
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

impl terrars::ToListMappable for CloudSecurityComplianceFramework_Timeouts {
    type O = terrars::BlockAssignable<CloudSecurityComplianceFramework_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFramework_Timeouts {}

impl BuildCloudSecurityComplianceFramework_Timeouts {
    pub fn build(self) -> CloudSecurityComplianceFramework_Timeouts {
        CloudSecurityComplianceFramework_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFramework_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceFramework_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudSecurityComplianceFramework_TimeoutsRef {
        CloudSecurityComplianceFramework_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFramework_TimeoutsRef {
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
