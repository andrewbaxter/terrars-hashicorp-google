#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CloudSecurityComplianceFrameworkDeploymentData_ {
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
    framework_deployment_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_control_metadata: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    framework: std::option::Option<std::vec::Vec<CloudSecurityComplianceFrameworkDeployment_Framework>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_resource_config: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudSecurityComplianceFrameworkDeployment_Timeouts>,
    dynamic: CloudSecurityComplianceFrameworkDeploymentDynamic,
}

struct CloudSecurityComplianceFrameworkDeployment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudSecurityComplianceFrameworkDeploymentData_>,
}

#[derive(Clone)]
pub struct CloudSecurityComplianceFrameworkDeployment(std::rc::Rc<CloudSecurityComplianceFrameworkDeployment_>);

impl CloudSecurityComplianceFrameworkDeployment {
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

    #[doc = "Set the field `description`.\nUser provided description of the Framework deployment"]
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
        "Set the field `location`.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `cloud_control_metadata`.\n"]
    pub fn set_cloud_control_metadata(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_control_metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_control_metadata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `framework`.\n"]
    pub fn set_framework(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudSecurityComplianceFrameworkDeployment_Framework>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().framework = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.framework = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_resource_config`.\n"]
    pub fn set_target_resource_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_resource_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_resource_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudSecurityComplianceFrameworkDeployment_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `cloud_control_deployment_references` after provisioning.\nThe references to the cloud control deployments. It has all the\nCloudControlDeployments which are either directly added in the framework or\nthrough a CloudControlGroup.\nExample: If a framework deployment deploys two\ncloud controls, cc-deployment-1 and cc-deployment-2, then the\ncloud_control_deployment_references will be:\n{\ncloud_control_deployment_reference: {\ncloud_control_deployment:\n\"organizations/{organization}/locations/{location}/cloudControlDeployments/cc-deployment-1\"\n},\ncloud_control_deployment_reference: {\ncloud_control_deployment:\n\"organizations/{organization}/locations/{location}/cloudControlDeployments/cc-deployment-2\"\n}"]
    pub fn cloud_control_deployment_references(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceFrameworkDeployment_CloudControlDeploymentReferencesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_control_deployment_references", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `computed_target_resource` after provisioning.\nThe resource on which the Framework is deployed based on the provided\nTargetResourceConfig in the following format:\norganizations/{organization}, folders/{folder} or projects/{project}"]
    pub fn computed_target_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.computed_target_resource", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployment_state` after provisioning.\nThe deployment state of the framework.\nPossible values:\nDEPLOYMENT_STATE_VALIDATING\nDEPLOYMENT_STATE_CREATING\nDEPLOYMENT_STATE_DELETING\nDEPLOYMENT_STATE_FAILED\nDEPLOYMENT_STATE_READY\nDEPLOYMENT_STATE_PARTIALLY_DEPLOYED\nDEPLOYMENT_STATE_PARTIALLY_DELETED"]
    pub fn deployment_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser provided description of the Framework deployment"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nTo prevent concurrent updates from overwriting each other, always provide\nthe 'etag' when you update a FrameworkDeployment. You can also\nprovide the 'etag' when you delete a FrameworkDeployment, to help\nensure that you're deleting the intended version of the\nFrameworkDeployment."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `framework_deployment_id` after provisioning.\nUser provided identifier. It should be unique in scope of a parent.\nThis is optional and if not provided, a random UUID will be generated."]
    pub fn framework_deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framework_deployment_id", self.extract_ref()))
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
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. FrameworkDeployment name in the following format:\n{parent}/locations/{location}/frameworkDeployments/{framework_deployment_id}"]
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
        "Get a reference to the value of field `target_resource_display_name` after provisioning.\nThe display name of the target resource."]
    pub fn target_resource_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_resource_display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the resource last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_control_metadata` after provisioning.\n"]
    pub fn cloud_control_metadata(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceFrameworkDeployment_CloudControlMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_control_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `framework` after provisioning.\n"]
    pub fn framework(&self) -> terrars::ListRef<CloudSecurityComplianceFrameworkDeployment_FrameworkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.framework", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_resource_config` after provisioning.\n"]
    pub fn target_resource_config(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceFrameworkDeployment_TargetResourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_resource_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudSecurityComplianceFrameworkDeployment_TimeoutsRef {
        CloudSecurityComplianceFrameworkDeployment_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for CloudSecurityComplianceFrameworkDeployment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudSecurityComplianceFrameworkDeployment { }

impl terrars::ToListMappable for CloudSecurityComplianceFrameworkDeployment {
    type O = terrars::ListRef<CloudSecurityComplianceFrameworkDeploymentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudSecurityComplianceFrameworkDeployment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_cloud_security_compliance_framework_deployment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudSecurityComplianceFrameworkDeployment {
    pub tf_id: std::string::String,
    #[doc =
        "User provided identifier. It should be unique in scope of a parent.\nThis is optional and if not provided, a random UUID will be generated."]
    pub framework_deployment_id: terrars::PrimField<std::string::String>,
}

impl BuildCloudSecurityComplianceFrameworkDeployment {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudSecurityComplianceFrameworkDeployment {
        let out =
            CloudSecurityComplianceFrameworkDeployment(std::rc::Rc::new(CloudSecurityComplianceFrameworkDeployment_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(CloudSecurityComplianceFrameworkDeploymentData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    provisioner: core::default::Default::default(),
                    deletion_policy: core::default::Default::default(),
                    description: core::default::Default::default(),
                    framework_deployment_id: self.framework_deployment_id,
                    id: core::default::Default::default(),
                    location: core::default::Default::default(),
                    organization: core::default::Default::default(),
                    parent: core::default::Default::default(),
                    cloud_control_metadata: core::default::Default::default(),
                    framework: core::default::Default::default(),
                    target_resource_config: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudSecurityComplianceFrameworkDeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceFrameworkDeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudSecurityComplianceFrameworkDeploymentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloud_control_deployment_references` after provisioning.\nThe references to the cloud control deployments. It has all the\nCloudControlDeployments which are either directly added in the framework or\nthrough a CloudControlGroup.\nExample: If a framework deployment deploys two\ncloud controls, cc-deployment-1 and cc-deployment-2, then the\ncloud_control_deployment_references will be:\n{\ncloud_control_deployment_reference: {\ncloud_control_deployment:\n\"organizations/{organization}/locations/{location}/cloudControlDeployments/cc-deployment-1\"\n},\ncloud_control_deployment_reference: {\ncloud_control_deployment:\n\"organizations/{organization}/locations/{location}/cloudControlDeployments/cc-deployment-2\"\n}"]
    pub fn cloud_control_deployment_references(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceFrameworkDeployment_CloudControlDeploymentReferencesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_control_deployment_references", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `computed_target_resource` after provisioning.\nThe resource on which the Framework is deployed based on the provided\nTargetResourceConfig in the following format:\norganizations/{organization}, folders/{folder} or projects/{project}"]
    pub fn computed_target_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.computed_target_resource", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployment_state` after provisioning.\nThe deployment state of the framework.\nPossible values:\nDEPLOYMENT_STATE_VALIDATING\nDEPLOYMENT_STATE_CREATING\nDEPLOYMENT_STATE_DELETING\nDEPLOYMENT_STATE_FAILED\nDEPLOYMENT_STATE_READY\nDEPLOYMENT_STATE_PARTIALLY_DEPLOYED\nDEPLOYMENT_STATE_PARTIALLY_DELETED"]
    pub fn deployment_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser provided description of the Framework deployment"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nTo prevent concurrent updates from overwriting each other, always provide\nthe 'etag' when you update a FrameworkDeployment. You can also\nprovide the 'etag' when you delete a FrameworkDeployment, to help\nensure that you're deleting the intended version of the\nFrameworkDeployment."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `framework_deployment_id` after provisioning.\nUser provided identifier. It should be unique in scope of a parent.\nThis is optional and if not provided, a random UUID will be generated."]
    pub fn framework_deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framework_deployment_id", self.extract_ref()))
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
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. FrameworkDeployment name in the following format:\n{parent}/locations/{location}/frameworkDeployments/{framework_deployment_id}"]
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
        "Get a reference to the value of field `target_resource_display_name` after provisioning.\nThe display name of the target resource."]
    pub fn target_resource_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_resource_display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the resource last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_control_metadata` after provisioning.\n"]
    pub fn cloud_control_metadata(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceFrameworkDeployment_CloudControlMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_control_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `framework` after provisioning.\n"]
    pub fn framework(&self) -> terrars::ListRef<CloudSecurityComplianceFrameworkDeployment_FrameworkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.framework", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_resource_config` after provisioning.\n"]
    pub fn target_resource_config(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceFrameworkDeployment_TargetResourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_resource_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudSecurityComplianceFrameworkDeployment_TimeoutsRef {
        CloudSecurityComplianceFrameworkDeployment_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkDeploymentDynamic {
    cloud_control_metadata: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata>>,
    framework: std::option::Option<terrars::DynamicBlock<CloudSecurityComplianceFrameworkDeployment_Framework>>,
    target_resource_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig>>,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeployment_CloudControlDeploymentReferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_control_deployment: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudSecurityComplianceFrameworkDeployment_CloudControlDeploymentReferences {
    #[doc = "Set the field `cloud_control_deployment`.\n"]
    pub fn set_cloud_control_deployment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloud_control_deployment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudSecurityComplianceFrameworkDeployment_CloudControlDeploymentReferences {
    type O = terrars::BlockAssignable<CloudSecurityComplianceFrameworkDeployment_CloudControlDeploymentReferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFrameworkDeployment_CloudControlDeploymentReferences {}

impl BuildCloudSecurityComplianceFrameworkDeployment_CloudControlDeploymentReferences {
    pub fn build(self) -> CloudSecurityComplianceFrameworkDeployment_CloudControlDeploymentReferences {
        CloudSecurityComplianceFrameworkDeployment_CloudControlDeploymentReferences {
            cloud_control_deployment: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFrameworkDeployment_CloudControlDeploymentReferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceFrameworkDeployment_CloudControlDeploymentReferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFrameworkDeployment_CloudControlDeploymentReferencesRef {
        CloudSecurityComplianceFrameworkDeployment_CloudControlDeploymentReferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFrameworkDeployment_CloudControlDeploymentReferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_control_deployment` after provisioning.\n"]
    pub fn cloud_control_deployment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_control_deployment", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValue {
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValue {

}

impl terrars
::ToListMappable for CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValue {
    type O =
        terrars
        ::BlockAssignable<
            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValue {
    #[doc = "The strings in the list."]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValue {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValue {
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValue {
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValueRef {
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValueRef {
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
struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValueDynamic {
    string_list_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue {
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
        std
        ::vec
        ::Vec<
            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValue,
        >,
    >,
    dynamic: CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValueDynamic,
}

impl CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue {
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
                            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValue,
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
::ToListMappable for CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue {
    type O =
        terrars
        ::BlockAssignable<
            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue {}

impl BuildCloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue {
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue {
            bool_value: core::default::Default::default(),
            number_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
            string_list_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValueRef {
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValueRef {
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
    ::ListRef<
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_list_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValueDynamic {
    parameter_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue,
        >,
    >,
    dynamic: CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValueDynamic,
}

impl CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue {
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
                            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValue,
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
::ToListMappable for CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue {
    type O =
        terrars
        ::BlockAssignable<
            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue {}

impl BuildCloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue {
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue {
            name: core::default::Default::default(),
            parameter_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValueRef {
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValueRef {
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
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue_ParameterValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameter_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValueDynamic {
    oneof_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue,
        >,
    >,
    string_list_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue {
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
        ::Vec<
            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_list_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValue,
        >,
    >,
    dynamic: CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValueDynamic,
}

impl CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue {
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
                            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValue,
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
                            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValue,
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
::ToListMappable for CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue {
    type O =
        terrars
        ::BlockAssignable<
            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue {}

impl BuildCloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue {
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue {
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
pub struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValueRef {
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValueRef {
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
    ::ListRef<
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_OneofValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oneof_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"]
    pub fn string_list_value(
        &self,
    ) -> terrars
    ::ListRef<
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue_StringListValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_list_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_ParametersDynamic {
    parameter_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue,
        >,
    >,
    dynamic: CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_ParametersDynamic,
}

impl CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters {
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
                            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValue,
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
::ToListMappable for CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters {
    type O =
        terrars
        ::BlockAssignable<
            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters {
    #[doc = "The name of the parameter."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters {
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters {
            name: self.name,
            parameter_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_ParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_ParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_ParametersRef {
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_ParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_ParametersRef {
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
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters_ParameterValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameter_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetailsDynamic {
    parameters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters>,
    >,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails {
    major_revision_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std
    ::option
    ::Option<
        std::vec::Vec<CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters>,
    >,
    dynamic: CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetailsDynamic,
}

impl CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_Parameters,
                        >,
                    >,
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

impl terrars::ToListMappable for CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails {
    type O =
        terrars
        ::BlockAssignable<CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails {
    #[doc = "Major revision of cloudcontrol"]
    pub major_revision_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the CloudControl in the format:\n\"{parent}/locations/{location}/cloudControls/{cloud-control}\""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails {
    pub fn build(self) -> CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails {
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails {
            major_revision_id: self.major_revision_id,
            name: self.name,
            parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetailsRef {
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetailsRef {
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
    pub fn parameters(
        &self,
    ) -> terrars
    ::ListRef<CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails_ParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadataDynamic {
    cloud_control_details: std
    ::option
    ::Option<
        terrars::DynamicBlock<CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails>,
    >,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata {
    enforcement_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_control_details: std
    ::option
    ::Option<std::vec::Vec<CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails>>,
    dynamic: CloudSecurityComplianceFrameworkDeployment_CloudControlMetadataDynamic,
}

impl CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata {
    #[doc = "Set the field `cloud_control_details`.\n"]
    pub fn set_cloud_control_details(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetails,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_control_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_control_details = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata {
    type O = terrars::BlockAssignable<CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFrameworkDeployment_CloudControlMetadata {
    #[doc = "Enforcement mode for the framework deployment.\nPossible values:\nPREVENTIVE\nDETECTIVE\nAUDIT"]
    pub enforcement_mode: terrars::PrimField<std::string::String>,
}

impl BuildCloudSecurityComplianceFrameworkDeployment_CloudControlMetadata {
    pub fn build(self) -> CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata {
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata {
            enforcement_mode: self.enforcement_mode,
            cloud_control_details: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFrameworkDeployment_CloudControlMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceFrameworkDeployment_CloudControlMetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFrameworkDeployment_CloudControlMetadataRef {
        CloudSecurityComplianceFrameworkDeployment_CloudControlMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFrameworkDeployment_CloudControlMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enforcement_mode` after provisioning.\nEnforcement mode for the framework deployment.\nPossible values:\nPREVENTIVE\nDETECTIVE\nAUDIT"]
    pub fn enforcement_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforcement_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_control_details` after provisioning.\n"]
    pub fn cloud_control_details(
        &self,
    ) -> terrars::ListRef<CloudSecurityComplianceFrameworkDeployment_CloudControlMetadata_CloudControlDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_control_details", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeployment_Framework {
    framework: terrars::PrimField<std::string::String>,
    major_revision_id: terrars::PrimField<std::string::String>,
}

impl CloudSecurityComplianceFrameworkDeployment_Framework { }

impl terrars::ToListMappable for CloudSecurityComplianceFrameworkDeployment_Framework {
    type O = terrars::BlockAssignable<CloudSecurityComplianceFrameworkDeployment_Framework>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFrameworkDeployment_Framework {
    #[doc = "In the format:\n{parent}/locations/{location}/frameworks/{framework}"]
    pub framework: terrars::PrimField<std::string::String>,
    #[doc = "Major revision id of the framework."]
    pub major_revision_id: terrars::PrimField<std::string::String>,
}

impl BuildCloudSecurityComplianceFrameworkDeployment_Framework {
    pub fn build(self) -> CloudSecurityComplianceFrameworkDeployment_Framework {
        CloudSecurityComplianceFrameworkDeployment_Framework {
            framework: self.framework,
            major_revision_id: self.major_revision_id,
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFrameworkDeployment_FrameworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceFrameworkDeployment_FrameworkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFrameworkDeployment_FrameworkRef {
        CloudSecurityComplianceFrameworkDeployment_FrameworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFrameworkDeployment_FrameworkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `framework` after provisioning.\nIn the format:\n{parent}/locations/{location}/frameworks/{framework}"]
    pub fn framework(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framework", self.base))
    }

    #[doc =
        "Get a reference to the value of field `major_revision_id` after provisioning.\nMajor revision id of the framework."]
    pub fn major_revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.major_revision_id", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_FolderCreationConfig {
    folder_display_name: terrars::PrimField<std::string::String>,
    parent: terrars::PrimField<std::string::String>,
}

impl CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_FolderCreationConfig {

}

impl terrars
::ToListMappable for CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_FolderCreationConfig {
    type O =
        terrars
        ::BlockAssignable<
            CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_FolderCreationConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_FolderCreationConfig {
    #[doc = "Display name of the folder to be created"]
    pub folder_display_name: terrars::PrimField<std::string::String>,
    #[doc = "The parent of the folder to be created. It can be an organizations/{org} or\nfolders/{folder}"]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildCloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_FolderCreationConfig {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_FolderCreationConfig {
        CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_FolderCreationConfig {
            folder_display_name: self.folder_display_name,
            parent: self.parent,
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_FolderCreationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_FolderCreationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_FolderCreationConfigRef {
        CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_FolderCreationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_FolderCreationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `folder_display_name` after provisioning.\nDisplay name of the folder to be created"]
    pub fn folder_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.folder_display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe parent of the folder to be created. It can be an organizations/{org} or\nfolders/{folder}"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_ProjectCreationConfig {
    billing_account_id: terrars::PrimField<std::string::String>,
    parent: terrars::PrimField<std::string::String>,
    project_display_name: terrars::PrimField<std::string::String>,
}

impl CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_ProjectCreationConfig {

}

impl terrars
::ToListMappable for CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_ProjectCreationConfig {
    type O =
        terrars
        ::BlockAssignable<
            CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_ProjectCreationConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_ProjectCreationConfig {
    #[doc = "Billing account id to be used for the project."]
    pub billing_account_id: terrars::PrimField<std::string::String>,
    #[doc = "organizations/{org} or folders/{folder}"]
    pub parent: terrars::PrimField<std::string::String>,
    #[doc = "Display name of the project to be created."]
    pub project_display_name: terrars::PrimField<std::string::String>,
}

impl BuildCloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_ProjectCreationConfig {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_ProjectCreationConfig {
        CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_ProjectCreationConfig {
            billing_account_id: self.billing_account_id,
            parent: self.parent,
            project_display_name: self.project_display_name,
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_ProjectCreationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_ProjectCreationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_ProjectCreationConfigRef {
        CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_ProjectCreationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_ProjectCreationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `billing_account_id` after provisioning.\nBilling account id to be used for the project."]
    pub fn billing_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_account_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\norganizations/{org} or folders/{folder}"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_display_name` after provisioning.\nDisplay name of the project to be created."]
    pub fn project_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_display_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfigDynamic {
    folder_creation_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_FolderCreationConfig,
        >,
    >,
    project_creation_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_ProjectCreationConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    folder_creation_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_FolderCreationConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_creation_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_ProjectCreationConfig,
        >,
    >,
    dynamic: CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfigDynamic,
}

impl CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig {
    #[doc = "Set the field `folder_creation_config`.\n"]
    pub fn set_folder_creation_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_FolderCreationConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.folder_creation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.folder_creation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `project_creation_config`.\n"]
    pub fn set_project_creation_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_ProjectCreationConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.project_creation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.project_creation_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig {
    type O =
        terrars
        ::BlockAssignable<
            CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig {}

impl BuildCloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig {
        CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig {
            folder_creation_config: core::default::Default::default(),
            project_creation_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfigRef {
        CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `folder_creation_config` after provisioning.\n"]
    pub fn folder_creation_config(
        &self,
    ) -> terrars
    ::ListRef<
        CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_FolderCreationConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.folder_creation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `project_creation_config` after provisioning.\n"]
    pub fn project_creation_config(
        &self,
    ) -> terrars
    ::ListRef<
        CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig_ProjectCreationConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.project_creation_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkDeployment_TargetResourceConfigDynamic {
    target_resource_creation_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig>,
    >,
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    existing_target_resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_resource_creation_config: std
    ::option
    ::Option<
        std::vec::Vec<CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig>,
    >,
    dynamic: CloudSecurityComplianceFrameworkDeployment_TargetResourceConfigDynamic,
}

impl CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig {
    #[doc =
        "Set the field `existing_target_resource`.\nCRM node in format organizations/{organization}, folders/{folder},\nor projects/{project}"]
    pub fn set_existing_target_resource(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.existing_target_resource = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_resource_creation_config`.\n"]
    pub fn set_target_resource_creation_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_resource_creation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_resource_creation_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig {
    type O = terrars::BlockAssignable<CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFrameworkDeployment_TargetResourceConfig {}

impl BuildCloudSecurityComplianceFrameworkDeployment_TargetResourceConfig {
    pub fn build(self) -> CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig {
        CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig {
            existing_target_resource: core::default::Default::default(),
            target_resource_creation_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFrameworkDeployment_TargetResourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceFrameworkDeployment_TargetResourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFrameworkDeployment_TargetResourceConfigRef {
        CloudSecurityComplianceFrameworkDeployment_TargetResourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFrameworkDeployment_TargetResourceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `existing_target_resource` after provisioning.\nCRM node in format organizations/{organization}, folders/{folder},\nor projects/{project}"]
    pub fn existing_target_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.existing_target_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `target_resource_creation_config` after provisioning.\n"]
    pub fn target_resource_creation_config(
        &self,
    ) -> terrars
    ::ListRef<CloudSecurityComplianceFrameworkDeployment_TargetResourceConfig_TargetResourceCreationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_resource_creation_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeployment_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudSecurityComplianceFrameworkDeployment_Timeouts {
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

impl terrars::ToListMappable for CloudSecurityComplianceFrameworkDeployment_Timeouts {
    type O = terrars::BlockAssignable<CloudSecurityComplianceFrameworkDeployment_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSecurityComplianceFrameworkDeployment_Timeouts {}

impl BuildCloudSecurityComplianceFrameworkDeployment_Timeouts {
    pub fn build(self) -> CloudSecurityComplianceFrameworkDeployment_Timeouts {
        CloudSecurityComplianceFrameworkDeployment_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudSecurityComplianceFrameworkDeployment_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSecurityComplianceFrameworkDeployment_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSecurityComplianceFrameworkDeployment_TimeoutsRef {
        CloudSecurityComplianceFrameworkDeployment_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSecurityComplianceFrameworkDeployment_TimeoutsRef {
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
