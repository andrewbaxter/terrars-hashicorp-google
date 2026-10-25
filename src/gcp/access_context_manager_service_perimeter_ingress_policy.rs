#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AccessContextManagerServicePerimeterIngressPolicyData_ {
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
    perimeter: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_from: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeterIngressPolicy_IngressFrom>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_to: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeterIngressPolicy_IngressTo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AccessContextManagerServicePerimeterIngressPolicy_Timeouts>,
    dynamic: AccessContextManagerServicePerimeterIngressPolicyDynamic,
}

struct AccessContextManagerServicePerimeterIngressPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AccessContextManagerServicePerimeterIngressPolicyData_>,
}

#[derive(Clone)]
pub struct AccessContextManagerServicePerimeterIngressPolicy(
    std::rc::Rc<AccessContextManagerServicePerimeterIngressPolicy_>,
);

impl AccessContextManagerServicePerimeterIngressPolicy {
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

    #[doc =
        "Set the field `title`.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn set_title(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().title = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ingress_from`.\n"]
    pub fn set_ingress_from(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicy_IngressFrom>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ingress_from = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ingress_from = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ingress_to`.\n"]
    pub fn set_ingress_to(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicy_IngressTo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ingress_to = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ingress_to = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl std::convert::Into<AccessContextManagerServicePerimeterIngressPolicy_Timeouts>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `access_policy_id` after provisioning.\nThe name of the Access Policy this resource belongs to."]
    pub fn access_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_policy_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe perimeter etag is internally used to prevent overwriting the list of policies on PATCH calls. It is retrieved from the same GET perimeter API call that's used to get the current list of policies. The policy defined in this resource is added or removed from that list, and then this etag is sent with the PATCH call along with the updated policies."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `perimeter` after provisioning.\nThe name of the Service Perimeter to add this resource to."]
    pub fn perimeter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.perimeter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress_from` after provisioning.\n"]
    pub fn ingress_from(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterIngressPolicy_IngressFromRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_from", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress_to` after provisioning.\n"]
    pub fn ingress_to(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterIngressPolicy_IngressToRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_to", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimeterIngressPolicy_TimeoutsRef {
        AccessContextManagerServicePerimeterIngressPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for AccessContextManagerServicePerimeterIngressPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AccessContextManagerServicePerimeterIngressPolicy { }

impl terrars::ToListMappable for AccessContextManagerServicePerimeterIngressPolicy {
    type O = terrars::ListRef<AccessContextManagerServicePerimeterIngressPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AccessContextManagerServicePerimeterIngressPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_access_context_manager_service_perimeter_ingress_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessContextManagerServicePerimeterIngressPolicy {
    pub tf_id: std::string::String,
    #[doc = "The name of the Service Perimeter to add this resource to."]
    pub perimeter: terrars::PrimField<std::string::String>,
}

impl BuildAccessContextManagerServicePerimeterIngressPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> AccessContextManagerServicePerimeterIngressPolicy {
        let out =
            AccessContextManagerServicePerimeterIngressPolicy(
                std::rc::Rc::new(AccessContextManagerServicePerimeterIngressPolicy_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(AccessContextManagerServicePerimeterIngressPolicyData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        provisioner: core::default::Default::default(),
                        deletion_policy: core::default::Default::default(),
                        id: core::default::Default::default(),
                        perimeter: self.perimeter,
                        title: core::default::Default::default(),
                        ingress_from: core::default::Default::default(),
                        ingress_to: core::default::Default::default(),
                        timeouts: core::default::Default::default(),
                        dynamic: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessContextManagerServicePerimeterIngressPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterIngressPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessContextManagerServicePerimeterIngressPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_policy_id` after provisioning.\nThe name of the Access Policy this resource belongs to."]
    pub fn access_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_policy_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe perimeter etag is internally used to prevent overwriting the list of policies on PATCH calls. It is retrieved from the same GET perimeter API call that's used to get the current list of policies. The policy defined in this resource is added or removed from that list, and then this etag is sent with the PATCH call along with the updated policies."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `perimeter` after provisioning.\nThe name of the Service Perimeter to add this resource to."]
    pub fn perimeter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.perimeter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress_from` after provisioning.\n"]
    pub fn ingress_from(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterIngressPolicy_IngressFromRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_from", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress_to` after provisioning.\n"]
    pub fn ingress_to(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterIngressPolicy_IngressToRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_to", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimeterIngressPolicy_TimeoutsRef {
        AccessContextManagerServicePerimeterIngressPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterIngressPolicyDynamic {
    ingress_from: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterIngressPolicy_IngressFrom>>,
    ingress_to: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterIngressPolicy_IngressTo>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources_PscEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarding_rule: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources_PscEndpoint {
    #[doc =
        "Set the field `forwarding_rule`.\nThe full resource name of the global forwarding rule that identifies a Private Service Connect endpoint.\nForwarding rule format: '//compute.googleapis.com/projects/{PROJECT_ID}/global/forwardingRules/{FORWARDING_RULE_ID}'."]
    pub fn set_forwarding_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.forwarding_rule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources_PscEndpoint {
    type O =
        terrars::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources_PscEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources_PscEndpoint {}

impl BuildAccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources_PscEndpoint {
    pub fn build(self) -> AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources_PscEndpoint {
        AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources_PscEndpoint {
            forwarding_rule: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources_PscEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources_PscEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources_PscEndpointRef {
        AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources_PscEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources_PscEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `forwarding_rule` after provisioning.\nThe full resource name of the global forwarding rule that identifies a Private Service Connect endpoint.\nForwarding rule format: '//compute.googleapis.com/projects/{PROJECT_ID}/global/forwardingRules/{FORWARDING_RULE_ID}'."]
    pub fn forwarding_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forwarding_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_SourcesDynamic {
    psc_endpoint: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources_PscEndpoint>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_endpoint: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources_PscEndpoint>>,
    dynamic: AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_SourcesDynamic,
}

impl AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources {
    #[doc =
        "Set the field `access_level`.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."]
    pub fn set_access_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_level = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource`.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nProjects, folders, organizations, and VPC networks are allowed.\nProject format: 'projects/{projectNumber}'\nFolder format: 'folders/{folderNumber}'\nOrganization format: 'organizations/{organizationNumber}'\nVPC network format:\n'//compute.googleapis.com/projects/{PROJECT_ID}/global/networks/{NAME}'.\nThe resource may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."]
    pub fn set_resource(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_endpoint`.\n"]
    pub fn set_psc_endpoint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources_PscEndpoint,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.psc_endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_endpoint = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources {}

impl BuildAccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources {
    pub fn build(self) -> AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources {
        AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources {
            access_level: core::default::Default::default(),
            resource: core::default::Default::default(),
            psc_endpoint: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_SourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_SourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_SourcesRef {
        AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_SourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_SourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_level` after provisioning.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."]
    pub fn access_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource` after provisioning.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nProjects, folders, organizations, and VPC networks are allowed.\nProject format: 'projects/{projectNumber}'\nFolder format: 'folders/{folderNumber}'\nOrganization format: 'organizations/{organizationNumber}'\nVPC network format:\n'//compute.googleapis.com/projects/{PROJECT_ID}/global/networks/{NAME}'.\nThe resource may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_endpoint` after provisioning.\n"]
    pub fn psc_endpoint(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources_PscEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterIngressPolicy_IngressFromDynamic {
    sources: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicy_IngressFrom {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identities: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sources: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources>>,
    dynamic: AccessContextManagerServicePerimeterIngressPolicy_IngressFromDynamic,
}

impl AccessContextManagerServicePerimeterIngressPolicy_IngressFrom {
    #[doc =
        "Set the field `identities`.\nIdentities can be an individual user, service account, Google group,\nor third-party identity. For third-party identity, only single identities\nare supported and other identity types are not supported.The v1 identities\nthat have the prefix user, group and serviceAccount in\nhttps://cloud.google.com/iam/docs/principal-identifiers#v1 are supported."]
    pub fn set_identities(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.identities = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_Sources>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterIngressPolicy_IngressFrom {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicy_IngressFrom>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterIngressPolicy_IngressFrom {}

impl BuildAccessContextManagerServicePerimeterIngressPolicy_IngressFrom {
    pub fn build(self) -> AccessContextManagerServicePerimeterIngressPolicy_IngressFrom {
        AccessContextManagerServicePerimeterIngressPolicy_IngressFrom {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeterIngressPolicy_IngressFromRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterIngressPolicy_IngressFromRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterIngressPolicy_IngressFromRef {
        AccessContextManagerServicePerimeterIngressPolicy_IngressFromRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterIngressPolicy_IngressFromRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identities` after provisioning.\nIdentities can be an individual user, service account, Google group,\nor third-party identity. For third-party identity, only single identities\nare supported and other identity types are not supported.The v1 identities\nthat have the prefix user, group and serviceAccount in\nhttps://cloud.google.com/iam/docs/principal-identifiers#v1 are supported."]
    pub fn identities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.base))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterIngressPolicy_IngressFrom_SourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations_MethodSelectors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations_MethodSelectors {
    #[doc =
        "Set the field `method`.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn set_permission(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.permission = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations_MethodSelectors {
    type O =
        terrars
        ::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations_MethodSelectors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations_MethodSelectors {}

impl BuildAccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations_MethodSelectors {
    pub fn build(self) -> AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations_MethodSelectors {
        AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations_MethodSelectors {
            method: core::default::Default::default(),
            permission: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations_MethodSelectorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations_MethodSelectorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations_MethodSelectorsRef {
        AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations_MethodSelectorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations_MethodSelectorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterIngressPolicy_IngressTo_OperationsDynamic {
    method_selectors: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations_MethodSelectors>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method_selectors: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations_MethodSelectors>>,
    dynamic: AccessContextManagerServicePerimeterIngressPolicy_IngressTo_OperationsDynamic,
}

impl AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations {
    #[doc =
        "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn set_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `method_selectors`.\n"]
    pub fn set_method_selectors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations_MethodSelectors,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.method_selectors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.method_selectors = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations {}

impl BuildAccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations {
    pub fn build(self) -> AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations {
        AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations {
            service_name: core::default::Default::default(),
            method_selectors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeterIngressPolicy_IngressTo_OperationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterIngressPolicy_IngressTo_OperationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterIngressPolicy_IngressTo_OperationsRef {
        AccessContextManagerServicePerimeterIngressPolicy_IngressTo_OperationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterIngressPolicy_IngressTo_OperationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"]
    pub fn method_selectors(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations_MethodSelectorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method_selectors", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterIngressPolicy_IngressToDynamic {
    operations: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicy_IngressTo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    roles: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations>>,
    dynamic: AccessContextManagerServicePerimeterIngressPolicy_IngressToDynamic,
}

impl AccessContextManagerServicePerimeterIngressPolicy_IngressTo {
    #[doc =
        "Set the field `resources`.\nA list of resources protected by this 'ServicePerimeter', in the form\n'projects/{project_number}', 'folders/{folder_number}', or\n'organizations/{organization_number}',\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `roles`.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'IngressFrom'\nare allowed to perform."]
    pub fn set_roles(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.roles = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicy_IngressTo_Operations>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterIngressPolicy_IngressTo {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicy_IngressTo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterIngressPolicy_IngressTo {}

impl BuildAccessContextManagerServicePerimeterIngressPolicy_IngressTo {
    pub fn build(self) -> AccessContextManagerServicePerimeterIngressPolicy_IngressTo {
        AccessContextManagerServicePerimeterIngressPolicy_IngressTo {
            resources: core::default::Default::default(),
            roles: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeterIngressPolicy_IngressToRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterIngressPolicy_IngressToRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterIngressPolicy_IngressToRef {
        AccessContextManagerServicePerimeterIngressPolicy_IngressToRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterIngressPolicy_IngressToRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of resources protected by this 'ServicePerimeter', in the form\n'projects/{project_number}', 'folders/{folder_number}', or\n'organizations/{organization_number}',\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn resources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `roles` after provisioning.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'IngressFrom'\nare allowed to perform."]
    pub fn roles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterIngressPolicy_IngressTo_OperationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeterIngressPolicy_Timeouts {
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

impl terrars::ToListMappable for AccessContextManagerServicePerimeterIngressPolicy_Timeouts {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterIngressPolicy_Timeouts {}

impl BuildAccessContextManagerServicePerimeterIngressPolicy_Timeouts {
    pub fn build(self) -> AccessContextManagerServicePerimeterIngressPolicy_Timeouts {
        AccessContextManagerServicePerimeterIngressPolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeterIngressPolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterIngressPolicy_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterIngressPolicy_TimeoutsRef {
        AccessContextManagerServicePerimeterIngressPolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterIngressPolicy_TimeoutsRef {
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
