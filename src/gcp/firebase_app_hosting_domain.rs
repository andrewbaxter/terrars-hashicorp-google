#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct FirebaseAppHostingDomainData_ {
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
    backend: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    domain_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serve: std::option::Option<std::vec::Vec<FirebaseAppHostingDomain_Serve>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FirebaseAppHostingDomain_Timeouts>,
    dynamic: FirebaseAppHostingDomainDynamic,
}

struct FirebaseAppHostingDomain_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FirebaseAppHostingDomainData_>,
}

#[derive(Clone)]
pub struct FirebaseAppHostingDomain(std::rc::Rc<FirebaseAppHostingDomain_>);

impl FirebaseAppHostingDomain {
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

    #[doc = "Set the field `serve`.\n"]
    pub fn set_serve(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirebaseAppHostingDomain_Serve>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().serve = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.serve = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FirebaseAppHostingDomain_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `backend` after provisioning.\nThe ID of the Backend that this Domain is associated with"]
    pub fn backend(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backend", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime at which the domain was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_domain_status` after provisioning.\nThe status of a custom domain's linkage to the Backend."]
    pub fn custom_domain_status(&self) -> terrars::ListRef<FirebaseAppHostingDomain_CustomDomainStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_domain_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nTime at which the domain was deleted."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain_id` after provisioning.\nId of the domain to create.\nMust be a valid domain name, such as \"foo.com\""]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nServer-computed checksum based on other values; may be sent\non update or delete to ensure operation is done on expected resource."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the Backend that this Domain is associated with"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the domain, e.g.\n'projects/{project}/locations/{locationId}/backends/{backendId}/domains/{domainId}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `purge_time` after provisioning.\nTime at which a soft-deleted domain will be purged, rendering in\npermanently deleted."]
    pub fn purge_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.purge_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-assigned, unique identifier."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime at which the domain was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serve` after provisioning.\n"]
    pub fn serve(&self) -> terrars::ListRef<FirebaseAppHostingDomain_ServeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serve", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppHostingDomain_TimeoutsRef {
        FirebaseAppHostingDomain_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for FirebaseAppHostingDomain {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FirebaseAppHostingDomain { }

impl terrars::ToListMappable for FirebaseAppHostingDomain {
    type O = terrars::ListRef<FirebaseAppHostingDomainRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FirebaseAppHostingDomain_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_firebase_app_hosting_domain".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFirebaseAppHostingDomain {
    pub tf_id: std::string::String,
    #[doc = "The ID of the Backend that this Domain is associated with"]
    pub backend: terrars::PrimField<std::string::String>,
    #[doc = "Id of the domain to create.\nMust be a valid domain name, such as \"foo.com\""]
    pub domain_id: terrars::PrimField<std::string::String>,
    #[doc = "The location of the Backend that this Domain is associated with"]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildFirebaseAppHostingDomain {
    pub fn build(self, stack: &mut terrars::Stack) -> FirebaseAppHostingDomain {
        let out = FirebaseAppHostingDomain(std::rc::Rc::new(FirebaseAppHostingDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FirebaseAppHostingDomainData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                backend: self.backend,
                deletion_policy: core::default::Default::default(),
                domain_id: self.domain_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                serve: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FirebaseAppHostingDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FirebaseAppHostingDomainRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backend` after provisioning.\nThe ID of the Backend that this Domain is associated with"]
    pub fn backend(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backend", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime at which the domain was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_domain_status` after provisioning.\nThe status of a custom domain's linkage to the Backend."]
    pub fn custom_domain_status(&self) -> terrars::ListRef<FirebaseAppHostingDomain_CustomDomainStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_domain_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nTime at which the domain was deleted."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain_id` after provisioning.\nId of the domain to create.\nMust be a valid domain name, such as \"foo.com\""]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nServer-computed checksum based on other values; may be sent\non update or delete to ensure operation is done on expected resource."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the Backend that this Domain is associated with"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the domain, e.g.\n'projects/{project}/locations/{locationId}/backends/{backendId}/domains/{domainId}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `purge_time` after provisioning.\nTime at which a soft-deleted domain will be purged, rendering in\npermanently deleted."]
    pub fn purge_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.purge_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-assigned, unique identifier."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime at which the domain was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serve` after provisioning.\n"]
    pub fn serve(&self) -> terrars::ListRef<FirebaseAppHostingDomain_ServeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serve", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppHostingDomain_TimeoutsRef {
        FirebaseAppHostingDomain_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct FirebaseAppHostingDomainDynamic {
    serve: std::option::Option<terrars::DynamicBlock<FirebaseAppHostingDomain_Serve>>,
}

#[derive(Serialize)]
pub struct FirebaseAppHostingDomain_CustomDomainStatus_Issues {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirebaseAppHostingDomain_CustomDomainStatus_Issues {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseAppHostingDomain_CustomDomainStatus_Issues {
    type O = terrars::BlockAssignable<FirebaseAppHostingDomain_CustomDomainStatus_Issues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingDomain_CustomDomainStatus_Issues {}

impl BuildFirebaseAppHostingDomain_CustomDomainStatus_Issues {
    pub fn build(self) -> FirebaseAppHostingDomain_CustomDomainStatus_Issues {
        FirebaseAppHostingDomain_CustomDomainStatus_Issues {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingDomain_CustomDomainStatus_IssuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingDomain_CustomDomainStatus_IssuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FirebaseAppHostingDomain_CustomDomainStatus_IssuesRef {
        FirebaseAppHostingDomain_CustomDomainStatus_IssuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingDomain_CustomDomainStatus_IssuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckError {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckError {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckError {
    type O =
        terrars::BlockAssignable<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckError>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckError {}

impl BuildFirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckError {
    pub fn build(self) -> FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckError {
        FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckError {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckErrorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckErrorRef {
        FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckErrorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_Records {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdata: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relevant_state: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_Records {
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rdata`.\n"]
    pub fn set_rdata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rdata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `relevant_state`.\n"]
    pub fn set_relevant_state(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.relevant_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `required_action`.\n"]
    pub fn set_required_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.required_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_Records {
    type O =
        terrars::BlockAssignable<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_Records>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_Records {}

impl BuildFirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_Records {
    pub fn build(self) -> FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_Records {
        FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_Records {
            domain_name: core::default::Default::default(),
            rdata: core::default::Default::default(),
            relevant_state: core::default::Default::default(),
            required_action: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_RecordsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_RecordsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_RecordsRef {
        FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_RecordsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_RecordsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `rdata` after provisioning.\n"]
    pub fn rdata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rdata", self.base))
    }

    #[doc = "Get a reference to the value of field `relevant_state` after provisioning.\n"]
    pub fn relevant_state(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.relevant_state", self.base))
    }

    #[doc = "Get a reference to the value of field `required_action` after provisioning.\n"]
    pub fn required_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required_action", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_error: std
    ::option
    ::Option<terrars::ListField<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckError>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    records: std
    ::option
    ::Option<terrars::ListField<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_Records>>,
}

impl FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired {
    #[doc = "Set the field `check_error`.\n"]
    pub fn set_check_error(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckError>,
                    >,
    ) -> Self {
        self.check_error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `records`.\n"]
    pub fn set_records(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_Records>,
                    >,
    ) -> Self {
        self.records = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired {
    type O = terrars::BlockAssignable<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired {}

impl BuildFirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired {
    pub fn build(self) -> FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired {
        FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired {
            check_error: core::default::Default::default(),
            domain_name: core::default::Default::default(),
            records: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_DesiredRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_DesiredRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_DesiredRef {
        FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_DesiredRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_DesiredRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `check_error` after provisioning.\n"]
    pub fn check_error(
        &self,
    ) -> terrars::ListRef<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check_error", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `records` after provisioning.\n"]
    pub fn records(
        &self,
    ) -> terrars::ListRef<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_RecordsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.records", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Discovered {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_error: std
    ::option
    ::Option<terrars::ListField<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckError>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    records: std
    ::option
    ::Option<terrars::ListField<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_Records>>,
}

impl FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Discovered {
    #[doc = "Set the field `check_error`.\n"]
    pub fn set_check_error(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckError>,
                    >,
    ) -> Self {
        self.check_error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `records`.\n"]
    pub fn set_records(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_Records>,
                    >,
    ) -> Self {
        self.records = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Discovered {
    type O = terrars::BlockAssignable<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Discovered>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Discovered {}

impl BuildFirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Discovered {
    pub fn build(self) -> FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Discovered {
        FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Discovered {
            check_error: core::default::Default::default(),
            domain_name: core::default::Default::default(),
            records: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_DiscoveredRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_DiscoveredRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_DiscoveredRef {
        FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_DiscoveredRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_DiscoveredRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `check_error` after provisioning.\n"]
    pub fn check_error(
        &self,
    ) -> terrars::ListRef<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_CheckErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check_error", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `records` after provisioning.\n"]
    pub fn records(
        &self,
    ) -> terrars::ListRef<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired_RecordsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.records", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired: std
    ::option
    ::Option<terrars::ListField<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discovered: std
    ::option
    ::Option<terrars::ListField<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Discovered>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates {
    #[doc = "Set the field `check_time`.\n"]
    pub fn set_check_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.check_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `desired`.\n"]
    pub fn set_desired(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Desired>,
                    >,
    ) -> Self {
        self.desired = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discovered`.\n"]
    pub fn set_discovered(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_Discovered>,
                    >,
    ) -> Self {
        self.discovered = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates {
    type O = terrars::BlockAssignable<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates {}

impl BuildFirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates {
    pub fn build(self) -> FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates {
        FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates {
            check_time: core::default::Default::default(),
            desired: core::default::Default::default(),
            discovered: core::default::Default::default(),
            domain_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdatesRef {
        FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdatesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `check_time` after provisioning.\n"]
    pub fn check_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.check_time", self.base))
    }

    #[doc = "Get a reference to the value of field `desired` after provisioning.\n"]
    pub fn desired(&self) -> terrars::ListRef<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_DesiredRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.desired", self.base))
    }

    #[doc = "Get a reference to the value of field `discovered` after provisioning.\n"]
    pub fn discovered(
        &self,
    ) -> terrars::ListRef<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates_DiscoveredRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discovered", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseAppHostingDomain_CustomDomainStatus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issues: std::option::Option<terrars::ListField<FirebaseAppHostingDomain_CustomDomainStatus_Issues>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ownership_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required_dns_updates: std
    ::option
    ::Option<terrars::ListField<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates>>,
}

impl FirebaseAppHostingDomain_CustomDomainStatus {
    #[doc = "Set the field `cert_state`.\n"]
    pub fn set_cert_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cert_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_state`.\n"]
    pub fn set_host_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issues`.\n"]
    pub fn set_issues(
        mut self,
        v: impl std::convert::Into<terrars::ListField<FirebaseAppHostingDomain_CustomDomainStatus_Issues>>,
    ) -> Self {
        self.issues = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ownership_state`.\n"]
    pub fn set_ownership_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ownership_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `required_dns_updates`.\n"]
    pub fn set_required_dns_updates(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdates>>,
    ) -> Self {
        self.required_dns_updates = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseAppHostingDomain_CustomDomainStatus {
    type O = terrars::BlockAssignable<FirebaseAppHostingDomain_CustomDomainStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingDomain_CustomDomainStatus {}

impl BuildFirebaseAppHostingDomain_CustomDomainStatus {
    pub fn build(self) -> FirebaseAppHostingDomain_CustomDomainStatus {
        FirebaseAppHostingDomain_CustomDomainStatus {
            cert_state: core::default::Default::default(),
            host_state: core::default::Default::default(),
            issues: core::default::Default::default(),
            ownership_state: core::default::Default::default(),
            required_dns_updates: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingDomain_CustomDomainStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingDomain_CustomDomainStatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FirebaseAppHostingDomain_CustomDomainStatusRef {
        FirebaseAppHostingDomain_CustomDomainStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingDomain_CustomDomainStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cert_state` after provisioning.\n"]
    pub fn cert_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cert_state", self.base))
    }

    #[doc = "Get a reference to the value of field `host_state` after provisioning.\n"]
    pub fn host_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_state", self.base))
    }

    #[doc = "Get a reference to the value of field `issues` after provisioning.\n"]
    pub fn issues(&self) -> terrars::ListRef<FirebaseAppHostingDomain_CustomDomainStatus_IssuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.issues", self.base))
    }

    #[doc = "Get a reference to the value of field `ownership_state` after provisioning.\n"]
    pub fn ownership_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ownership_state", self.base))
    }

    #[doc = "Get a reference to the value of field `required_dns_updates` after provisioning.\n"]
    pub fn required_dns_updates(
        &self,
    ) -> terrars::ListRef<FirebaseAppHostingDomain_CustomDomainStatus_RequiredDnsUpdatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.required_dns_updates", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseAppHostingDomain_Serve_Redirect {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
}

impl FirebaseAppHostingDomain_Serve_Redirect {
    #[doc =
        "Set the field `status`.\nThe status code to use in a redirect response. Must be a valid HTTP 3XX\nstatus code. Defaults to 302 if not present."]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseAppHostingDomain_Serve_Redirect {
    type O = terrars::BlockAssignable<FirebaseAppHostingDomain_Serve_Redirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingDomain_Serve_Redirect {
    #[doc =
        "The URI of the redirect's intended destination. This URI will be\nprepended to the original request path. URI without a scheme are\nassumed to be HTTPS."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildFirebaseAppHostingDomain_Serve_Redirect {
    pub fn build(self) -> FirebaseAppHostingDomain_Serve_Redirect {
        FirebaseAppHostingDomain_Serve_Redirect {
            status: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingDomain_Serve_RedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingDomain_Serve_RedirectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaseAppHostingDomain_Serve_RedirectRef {
        FirebaseAppHostingDomain_Serve_RedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingDomain_Serve_RedirectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status code to use in a redirect response. Must be a valid HTTP 3XX\nstatus code. Defaults to 302 if not present."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nThe URI of the redirect's intended destination. This URI will be\nprepended to the original request path. URI without a scheme are\nassumed to be HTTPS."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct FirebaseAppHostingDomain_ServeDynamic {
    redirect: std::option::Option<terrars::DynamicBlock<FirebaseAppHostingDomain_Serve_Redirect>>,
}

#[derive(Serialize)]
pub struct FirebaseAppHostingDomain_Serve {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<std::vec::Vec<FirebaseAppHostingDomain_Serve_Redirect>>,
    dynamic: FirebaseAppHostingDomain_ServeDynamic,
}

impl FirebaseAppHostingDomain_Serve {
    #[doc = "Set the field `redirect`.\n"]
    pub fn set_redirect(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirebaseAppHostingDomain_Serve_Redirect>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redirect = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redirect = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FirebaseAppHostingDomain_Serve {
    type O = terrars::BlockAssignable<FirebaseAppHostingDomain_Serve>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingDomain_Serve {}

impl BuildFirebaseAppHostingDomain_Serve {
    pub fn build(self) -> FirebaseAppHostingDomain_Serve {
        FirebaseAppHostingDomain_Serve {
            redirect: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingDomain_ServeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingDomain_ServeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaseAppHostingDomain_ServeRef {
        FirebaseAppHostingDomain_ServeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingDomain_ServeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> terrars::ListRef<FirebaseAppHostingDomain_Serve_RedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseAppHostingDomain_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirebaseAppHostingDomain_Timeouts {
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

impl terrars::ToListMappable for FirebaseAppHostingDomain_Timeouts {
    type O = terrars::BlockAssignable<FirebaseAppHostingDomain_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingDomain_Timeouts {}

impl BuildFirebaseAppHostingDomain_Timeouts {
    pub fn build(self) -> FirebaseAppHostingDomain_Timeouts {
        FirebaseAppHostingDomain_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingDomain_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingDomain_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaseAppHostingDomain_TimeoutsRef {
        FirebaseAppHostingDomain_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingDomain_TimeoutsRef {
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
