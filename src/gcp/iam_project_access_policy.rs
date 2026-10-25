#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IamProjectAccessPolicyData_ {
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
    access_policy_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<std::vec::Vec<IamProjectAccessPolicy_Details>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IamProjectAccessPolicy_Timeouts>,
    dynamic: IamProjectAccessPolicyDynamic,
}

struct IamProjectAccessPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IamProjectAccessPolicyData_>,
}

#[derive(Clone)]
pub struct IamProjectAccessPolicy(std::rc::Rc<IamProjectAccessPolicy_>);

impl IamProjectAccessPolicy {
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
        "Set the field `annotations`.\nUser defined annotations. See https://google.aip.dev/148#annotations for\nmore details such as format and size limitations\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
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
        "Set the field `display_name`.\nThe description of the access policy. Must be less than\nor equal to 63 characters."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamProjectAccessPolicy_Details>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<IamProjectAccessPolicy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `access_policy_id` after provisioning.\nThe ID to use for the access policy, which\nwill become the final component of the access policy's\nresource name.\nThis value must start with a lowercase letter followed by up to 62\nlowercase letters, numbers, hyphens, or dots. Pattern,\n/a-z{2,62}/.\nThis value must be unique among all access policies with the same parent."]
    pub fn access_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_policy_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUser defined annotations. See https://google.aip.dev/148#annotations for\nmore details such as format and size limitations\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the access policy was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe description of the access policy. Must be less than\nor equal to 63 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe etag for the access policy.\nIf this is provided on update, it must match the server's etag."]
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
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the access policy.\nThe following formats are supported:\n* 'projects/{project_id}/locations/{location}/accessPolicies/{policy_id}'\n* 'projects/{project_number}/locations/{location}/accessPolicies/{policy_id}'\n* 'folders/{folder_id}/locations/{location}/accessPolicies/{policy_id}'\n* 'organizations/{organization_id}/locations/{location}/accessPolicies/{policy_id}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe globally unique ID of the access policy."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the access policy was most recently\nupdated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<IamProjectAccessPolicy_DetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamProjectAccessPolicy_TimeoutsRef {
        IamProjectAccessPolicy_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for IamProjectAccessPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IamProjectAccessPolicy { }

impl terrars::ToListMappable for IamProjectAccessPolicy {
    type O = terrars::ListRef<IamProjectAccessPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IamProjectAccessPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_iam_project_access_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIamProjectAccessPolicy {
    pub tf_id: std::string::String,
    #[doc =
        "The ID to use for the access policy, which\nwill become the final component of the access policy's\nresource name.\nThis value must start with a lowercase letter followed by up to 62\nlowercase letters, numbers, hyphens, or dots. Pattern,\n/a-z{2,62}/.\nThis value must be unique among all access policies with the same parent."]
    pub access_policy_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildIamProjectAccessPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> IamProjectAccessPolicy {
        let out = IamProjectAccessPolicy(std::rc::Rc::new(IamProjectAccessPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IamProjectAccessPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                access_policy_id: self.access_policy_id,
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                details: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IamProjectAccessPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamProjectAccessPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IamProjectAccessPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_policy_id` after provisioning.\nThe ID to use for the access policy, which\nwill become the final component of the access policy's\nresource name.\nThis value must start with a lowercase letter followed by up to 62\nlowercase letters, numbers, hyphens, or dots. Pattern,\n/a-z{2,62}/.\nThis value must be unique among all access policies with the same parent."]
    pub fn access_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_policy_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUser defined annotations. See https://google.aip.dev/148#annotations for\nmore details such as format and size limitations\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the access policy was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe description of the access policy. Must be less than\nor equal to 63 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe etag for the access policy.\nIf this is provided on update, it must match the server's etag."]
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
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the access policy.\nThe following formats are supported:\n* 'projects/{project_id}/locations/{location}/accessPolicies/{policy_id}'\n* 'projects/{project_number}/locations/{location}/accessPolicies/{policy_id}'\n* 'folders/{folder_id}/locations/{location}/accessPolicies/{policy_id}'\n* 'organizations/{organization_id}/locations/{location}/accessPolicies/{policy_id}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe globally unique ID of the access policy."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the access policy was most recently\nupdated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<IamProjectAccessPolicy_DetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamProjectAccessPolicy_TimeoutsRef {
        IamProjectAccessPolicy_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct IamProjectAccessPolicyDynamic {
    details: std::option::Option<terrars::DynamicBlock<IamProjectAccessPolicy_Details>>,
}

#[derive(Serialize)]
pub struct IamProjectAccessPolicy_Details_Rules_Conditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    service: terrars::PrimField<std::string::String>,
}

impl IamProjectAccessPolicy_Details_Rules_Conditions {
    #[doc =
        "Set the field `expression`.\nTextual representation of an expression in Common Expression Language\nsyntax."]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IamProjectAccessPolicy_Details_Rules_Conditions {
    type O = terrars::BlockAssignable<IamProjectAccessPolicy_Details_Rules_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamProjectAccessPolicy_Details_Rules_Conditions {
    #[doc = ""]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildIamProjectAccessPolicy_Details_Rules_Conditions {
    pub fn build(self) -> IamProjectAccessPolicy_Details_Rules_Conditions {
        IamProjectAccessPolicy_Details_Rules_Conditions {
            expression: core::default::Default::default(),
            service: self.service,
        }
    }
}

#[allow(dead_code)]
pub struct IamProjectAccessPolicy_Details_Rules_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamProjectAccessPolicy_Details_Rules_ConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamProjectAccessPolicy_Details_Rules_ConditionsRef {
        IamProjectAccessPolicy_Details_Rules_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamProjectAccessPolicy_Details_Rules_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language\nsyntax."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct IamProjectAccessPolicy_Details_Rules_Operation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_permissions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    permissions: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl IamProjectAccessPolicy_Details_Rules_Operation {
    #[doc =
        "Set the field `excluded_permissions`.\nSpecifies the permissions that this rule excludes from the set of\naffected permissions given by 'permissions'. If a permission appears in\n'permissions' _and_ in 'excluded_permissions' then it will _not_ be\nsubject to the policy effect.\nThe excluded permissions can be specified using the same syntax as\n'permissions'."]
    pub fn set_excluded_permissions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_permissions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IamProjectAccessPolicy_Details_Rules_Operation {
    type O = terrars::BlockAssignable<IamProjectAccessPolicy_Details_Rules_Operation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamProjectAccessPolicy_Details_Rules_Operation {
    #[doc =
        "The permissions that are explicitly affected by this rule. Each\npermission uses the format '{service_fqdn}/{resource}.{verb}', where\n'{service_fqdn}' is the fully qualified domain name for the service.\nCurrently supported permissions are as follows:\n* 'eventarc.googleapis.com/messageBuses.publish'."]
    pub permissions: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildIamProjectAccessPolicy_Details_Rules_Operation {
    pub fn build(self) -> IamProjectAccessPolicy_Details_Rules_Operation {
        IamProjectAccessPolicy_Details_Rules_Operation {
            excluded_permissions: core::default::Default::default(),
            permissions: self.permissions,
        }
    }
}

#[allow(dead_code)]
pub struct IamProjectAccessPolicy_Details_Rules_OperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamProjectAccessPolicy_Details_Rules_OperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamProjectAccessPolicy_Details_Rules_OperationRef {
        IamProjectAccessPolicy_Details_Rules_OperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamProjectAccessPolicy_Details_Rules_OperationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `excluded_permissions` after provisioning.\nSpecifies the permissions that this rule excludes from the set of\naffected permissions given by 'permissions'. If a permission appears in\n'permissions' _and_ in 'excluded_permissions' then it will _not_ be\nsubject to the policy effect.\nThe excluded permissions can be specified using the same syntax as\n'permissions'."]
    pub fn excluded_permissions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_permissions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permissions` after provisioning.\nThe permissions that are explicitly affected by this rule. Each\npermission uses the format '{service_fqdn}/{resource}.{verb}', where\n'{service_fqdn}' is the fully qualified domain name for the service.\nCurrently supported permissions are as follows:\n* 'eventarc.googleapis.com/messageBuses.publish'."]
    pub fn permissions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }
}

#[derive(Serialize, Default)]
struct IamProjectAccessPolicy_Details_RulesDynamic {
    conditions: std::option::Option<terrars::DynamicBlock<IamProjectAccessPolicy_Details_Rules_Conditions>>,
    operation: std::option::Option<terrars::DynamicBlock<IamProjectAccessPolicy_Details_Rules_Operation>>,
}

#[derive(Serialize)]
pub struct IamProjectAccessPolicy_Details_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    effect: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_principals: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    principals: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<std::vec::Vec<IamProjectAccessPolicy_Details_Rules_Conditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation: std::option::Option<std::vec::Vec<IamProjectAccessPolicy_Details_Rules_Operation>>,
    dynamic: IamProjectAccessPolicy_Details_RulesDynamic,
}

impl IamProjectAccessPolicy_Details_Rules {
    #[doc =
        "Set the field `description`.\nCustomer specified description of the rule. Must be less than or equal to\n256 characters."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `excluded_principals`.\nThe identities that are excluded from the access policy rule, even if they\nare listed in the 'principals'. For example, you could add a Google\ngroup to the 'principals', then exclude specific users who belong to\nthat group."]
    pub fn set_excluded_principals(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_principals = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamProjectAccessPolicy_Details_Rules_Conditions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `operation`.\n"]
    pub fn set_operation(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamProjectAccessPolicy_Details_Rules_Operation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IamProjectAccessPolicy_Details_Rules {
    type O = terrars::BlockAssignable<IamProjectAccessPolicy_Details_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamProjectAccessPolicy_Details_Rules {
    #[doc = "The effect of the rule.\nPossible values:\nDENY\nALLOW Possible values: [\"DENY\", \"ALLOW\"]"]
    pub effect: terrars::PrimField<std::string::String>,
    #[doc =
        "The identities for which this rule's effect governs using one or more\npermissions on Google Cloud resources. This field can contain the\nfollowing values:\n* 'principal://goog/subject/{email_id}': A specific Google Account.\nIncludes Gmail, Cloud Identity, and Google Workspace user accounts. For\nexample, 'principal://goog/subject/alice@example.com'.\n* 'principal://iam.googleapis.com/projects/-/serviceAccounts/{service_account_id}':\nA Google Cloud service account. For example,\n'principal://iam.googleapis.com/projects/-/serviceAccounts/my-service-account@iam.gserviceaccount.com'.\n* 'principalSet://goog/group/{group_id}': A Google group. For example,\n'principalSet://goog/group/admins@example.com'.\n* 'principalSet://goog/cloudIdentityCustomerId/{customer_id}': All of the\nprincipals associated with the specified Google Workspace or Cloud\nIdentity customer ID. For example,\n'principalSet://goog/cloudIdentityCustomerId/C01Abc35'.\nIf an identifier that was previously set on a policy is soft deleted, then\ncalls to read that policy will return the identifier with a deleted\nprefix. Users cannot set identifiers with this syntax.\n* 'deleted:principal://goog/subject/{email_id}?uid={uid}': A specific\nGoogle Account that was deleted recently. For example,\n'deleted:principal://goog/subject/alice@example.com?uid=1234567890'. If\nthe Google Account is recovered, this identifier reverts to the standard\nidentifier for a Google Account.\n* 'deleted:principalSet://goog/group/{group_id}?uid={uid}': A Google group\nthat was deleted recently. For example,\n'deleted:principalSet://goog/group/admins@example.com?uid=1234567890'.\nIf the Google group is restored, this identifier reverts to the standard\nidentifier for a Google group.\n* 'deleted:principal://iam.googleapis.com/projects/-/serviceAccounts/{service_account_id}?uid={uid}':\nA Google Cloud service account that was deleted recently. For example,\n'deleted:principal://iam.googleapis.com/projects/-/serviceAccounts/my-service-account@iam.gserviceaccount.com?uid=1234567890'.\nIf the service account is undeleted, this identifier reverts to the\nstandard identifier for a service account."]
    pub principals: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildIamProjectAccessPolicy_Details_Rules {
    pub fn build(self) -> IamProjectAccessPolicy_Details_Rules {
        IamProjectAccessPolicy_Details_Rules {
            description: core::default::Default::default(),
            effect: self.effect,
            excluded_principals: core::default::Default::default(),
            principals: self.principals,
            conditions: core::default::Default::default(),
            operation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamProjectAccessPolicy_Details_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamProjectAccessPolicy_Details_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamProjectAccessPolicy_Details_RulesRef {
        IamProjectAccessPolicy_Details_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamProjectAccessPolicy_Details_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nCustomer specified description of the rule. Must be less than or equal to\n256 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `effect` after provisioning.\nThe effect of the rule.\nPossible values:\nDENY\nALLOW Possible values: [\"DENY\", \"ALLOW\"]"]
    pub fn effect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `excluded_principals` after provisioning.\nThe identities that are excluded from the access policy rule, even if they\nare listed in the 'principals'. For example, you could add a Google\ngroup to the 'principals', then exclude specific users who belong to\nthat group."]
    pub fn excluded_principals(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_principals", self.base))
    }

    #[doc =
        "Get a reference to the value of field `principals` after provisioning.\nThe identities for which this rule's effect governs using one or more\npermissions on Google Cloud resources. This field can contain the\nfollowing values:\n* 'principal://goog/subject/{email_id}': A specific Google Account.\nIncludes Gmail, Cloud Identity, and Google Workspace user accounts. For\nexample, 'principal://goog/subject/alice@example.com'.\n* 'principal://iam.googleapis.com/projects/-/serviceAccounts/{service_account_id}':\nA Google Cloud service account. For example,\n'principal://iam.googleapis.com/projects/-/serviceAccounts/my-service-account@iam.gserviceaccount.com'.\n* 'principalSet://goog/group/{group_id}': A Google group. For example,\n'principalSet://goog/group/admins@example.com'.\n* 'principalSet://goog/cloudIdentityCustomerId/{customer_id}': All of the\nprincipals associated with the specified Google Workspace or Cloud\nIdentity customer ID. For example,\n'principalSet://goog/cloudIdentityCustomerId/C01Abc35'.\nIf an identifier that was previously set on a policy is soft deleted, then\ncalls to read that policy will return the identifier with a deleted\nprefix. Users cannot set identifiers with this syntax.\n* 'deleted:principal://goog/subject/{email_id}?uid={uid}': A specific\nGoogle Account that was deleted recently. For example,\n'deleted:principal://goog/subject/alice@example.com?uid=1234567890'. If\nthe Google Account is recovered, this identifier reverts to the standard\nidentifier for a Google Account.\n* 'deleted:principalSet://goog/group/{group_id}?uid={uid}': A Google group\nthat was deleted recently. For example,\n'deleted:principalSet://goog/group/admins@example.com?uid=1234567890'.\nIf the Google group is restored, this identifier reverts to the standard\nidentifier for a Google group.\n* 'deleted:principal://iam.googleapis.com/projects/-/serviceAccounts/{service_account_id}?uid={uid}':\nA Google Cloud service account that was deleted recently. For example,\n'deleted:principal://iam.googleapis.com/projects/-/serviceAccounts/my-service-account@iam.gserviceaccount.com?uid=1234567890'.\nIf the service account is undeleted, this identifier reverts to the\nstandard identifier for a service account."]
    pub fn principals(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principals", self.base))
    }

    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> terrars::ListRef<IamProjectAccessPolicy_Details_Rules_OperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize, Default)]
struct IamProjectAccessPolicy_DetailsDynamic {
    rules: std::option::Option<terrars::DynamicBlock<IamProjectAccessPolicy_Details_Rules>>,
}

#[derive(Serialize)]
pub struct IamProjectAccessPolicy_Details {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<IamProjectAccessPolicy_Details_Rules>>,
    dynamic: IamProjectAccessPolicy_DetailsDynamic,
}

impl IamProjectAccessPolicy_Details {
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamProjectAccessPolicy_Details_Rules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IamProjectAccessPolicy_Details {
    type O = terrars::BlockAssignable<IamProjectAccessPolicy_Details>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamProjectAccessPolicy_Details {}

impl BuildIamProjectAccessPolicy_Details {
    pub fn build(self) -> IamProjectAccessPolicy_Details {
        IamProjectAccessPolicy_Details {
            rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamProjectAccessPolicy_DetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamProjectAccessPolicy_DetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamProjectAccessPolicy_DetailsRef {
        IamProjectAccessPolicy_DetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamProjectAccessPolicy_DetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<IamProjectAccessPolicy_Details_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}

#[derive(Serialize)]
pub struct IamProjectAccessPolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IamProjectAccessPolicy_Timeouts {
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

impl terrars::ToListMappable for IamProjectAccessPolicy_Timeouts {
    type O = terrars::BlockAssignable<IamProjectAccessPolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamProjectAccessPolicy_Timeouts {}

impl BuildIamProjectAccessPolicy_Timeouts {
    pub fn build(self) -> IamProjectAccessPolicy_Timeouts {
        IamProjectAccessPolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamProjectAccessPolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamProjectAccessPolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamProjectAccessPolicy_TimeoutsRef {
        IamProjectAccessPolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamProjectAccessPolicy_TimeoutsRef {
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
