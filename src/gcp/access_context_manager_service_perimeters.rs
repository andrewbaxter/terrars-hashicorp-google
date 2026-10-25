#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AccessContextManagerServicePerimetersData_ {
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
    parent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_perimeters: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeters_ServicePerimeters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AccessContextManagerServicePerimeters_Timeouts>,
    dynamic: AccessContextManagerServicePerimetersDynamic,
}

struct AccessContextManagerServicePerimeters_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AccessContextManagerServicePerimetersData_>,
}

#[derive(Clone)]
pub struct AccessContextManagerServicePerimeters(std::rc::Rc<AccessContextManagerServicePerimeters_>);

impl AccessContextManagerServicePerimeters {
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

    #[doc = "Set the field `service_perimeters`.\n"]
    pub fn set_service_perimeters(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessContextManagerServicePerimeters_ServicePerimeters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_perimeters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_perimeters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AccessContextManagerServicePerimeters_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe AccessPolicy this ServicePerimeter lives in.\nFormat: accessPolicies/{policy_id}"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_perimeters` after provisioning.\n"]
    pub fn service_perimeters(&self) -> terrars::ListRef<AccessContextManagerServicePerimeters_ServicePerimetersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_perimeters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimeters_TimeoutsRef {
        AccessContextManagerServicePerimeters_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for AccessContextManagerServicePerimeters {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AccessContextManagerServicePerimeters { }

impl terrars::ToListMappable for AccessContextManagerServicePerimeters {
    type O = terrars::ListRef<AccessContextManagerServicePerimetersRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AccessContextManagerServicePerimeters_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_access_context_manager_service_perimeters".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessContextManagerServicePerimeters {
    pub tf_id: std::string::String,
    #[doc = "The AccessPolicy this ServicePerimeter lives in.\nFormat: accessPolicies/{policy_id}"]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildAccessContextManagerServicePerimeters {
    pub fn build(self, stack: &mut terrars::Stack) -> AccessContextManagerServicePerimeters {
        let out = AccessContextManagerServicePerimeters(std::rc::Rc::new(AccessContextManagerServicePerimeters_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AccessContextManagerServicePerimetersData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                parent: self.parent,
                service_perimeters: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessContextManagerServicePerimetersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessContextManagerServicePerimetersRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe AccessPolicy this ServicePerimeter lives in.\nFormat: accessPolicies/{policy_id}"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_perimeters` after provisioning.\n"]
    pub fn service_perimeters(&self) -> terrars::ListRef<AccessContextManagerServicePerimeters_ServicePerimetersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_perimeters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimeters_TimeoutsRef {
        AccessContextManagerServicePerimeters_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersDynamic {
    service_perimeters: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeters_ServicePerimeters>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarding_rule: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpoint {
    #[doc =
        "Set the field `forwarding_rule`.\nThe full resource name of the global forwarding rule that identifies a Private Service Connect endpoint.\nForwarding rule format: '//compute.googleapis.com/projects/{PROJECT_ID}/global/forwardingRules/{FORWARDING_RULE_ID}'."]
    pub fn set_forwarding_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.forwarding_rule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpoint {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpoint,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpoint {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpoint {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpoint {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpoint {
            forwarding_rule: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpointRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpointRef {
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
struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_SourcesDynamic {
    psc_endpoint: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpoint,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_endpoint: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpoint,
        >,
    >,
    dynamic: AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_SourcesDynamic,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources {
    #[doc =
        "Set the field `access_level`.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."]
    pub fn set_access_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_level = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource`.\nA Google Cloud resource that is allowed to egress the perimeter.\nRequests from these resources are allowed to access data outside the perimeter.\nProjects, folders, and organizations are allowed.\nProject format: 'projects/{project_number}'\nFolder format: 'folders/{folder_number}'\nOrganization format: 'organizations/{organization_number}'\nThe resource may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the\ncase of allowing all Google Cloud resources only is not supported."]
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
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpoint,
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

impl terrars
::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources {
            access_level: core::default::Default::default(),
            resource: core::default::Default::default(),
            psc_endpoint: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_SourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_SourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_SourcesRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_SourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_SourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_level` after provisioning.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."]
    pub fn access_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource` after provisioning.\nA Google Cloud resource that is allowed to egress the perimeter.\nRequests from these resources are allowed to access data outside the perimeter.\nProjects, folders, and organizations are allowed.\nProject format: 'projects/{project_number}'\nFolder format: 'folders/{folder_number}'\nOrganization format: 'organizations/{organization_number}'\nThe resource may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the\ncase of allowing all Google Cloud resources only is not supported."]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_endpoint` after provisioning.\n"]
    pub fn psc_endpoint(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpointRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFromDynamic {
    sources: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_restriction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sources: std
    ::option
    ::Option<
        std::vec::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources>,
    >,
    dynamic: AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFromDynamic,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom {
    #[doc =
        "Set the field `identities`.\nIdentities can be an individual user, service account, Google group,\nor third-party identity. For third-party identity, only single identities\nare supported and other identity types are not supported.The v1 identities\nthat have the prefix user, group and serviceAccount in\nhttps://cloud.google.com/iam/docs/principal-identifiers#v1 are supported."]
    pub fn set_identities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.identities = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_restriction`.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn set_source_restriction(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_restriction = std::option::Option::Some(v.into());
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
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources,
                        >,
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

impl terrars
::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom {
    type O =
        terrars
        ::BlockAssignable<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom {
    pub fn build(self) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            source_restriction: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFromRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFromRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFromRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFromRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFromRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identities` after provisioning.\nIdentities can be an individual user, service account, Google group,\nor third-party identity. For third-party identity, only single identities\nare supported and other identity types are not supported.The v1 identities\nthat have the prefix user, group and serviceAccount in\nhttps://cloud.google.com/iam/docs/principal-identifiers#v1 are supported."]
    pub fn identities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_restriction` after provisioning.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn source_restriction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_restriction", self.base))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_SourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations_MethodSelectors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations_MethodSelectors {
    #[doc =
        "Set the field `method`.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."]
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
::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations_MethodSelectors {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations_MethodSelectors,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations_MethodSelectors {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations_MethodSelectors {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations_MethodSelectors {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations_MethodSelectors {
            method: core::default::Default::default(),
            permission: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations_MethodSelectorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations_MethodSelectorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations_MethodSelectorsRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations_MethodSelectorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations_MethodSelectorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."]
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
struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_OperationsDynamic {
    method_selectors: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations_MethodSelectors,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method_selectors: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations_MethodSelectors,
        >,
    >,
    dynamic: AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_OperationsDynamic,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations {
    #[doc =
        "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."]
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
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations_MethodSelectors,
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

impl terrars
::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations {
            service_name: core::default::Default::default(),
            method_selectors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_OperationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_OperationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_OperationsRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_OperationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_OperationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"]
    pub fn method_selectors(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations_MethodSelectorsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method_selectors", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressToDynamic {
    operations: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    roles: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations: std
    ::option
    ::Option<
        std::vec::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations>,
    >,
    dynamic: AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressToDynamic,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo {
    #[doc =
        "Set the field `external_resources`.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn set_external_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.external_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resources`.\nA list of resources, in the form 'projects/{project_number}',\n'folders/{folder_number}', or 'organizations/{organization_number}',\nthat match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `roles`.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'EgressFrom'\nare allowed to perform."]
    pub fn set_roles(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
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
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_Operations,
                        >,
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

impl terrars::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo {
    type O =
        terrars
        ::BlockAssignable<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo {
    pub fn build(self) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo {
            external_resources: core::default::Default::default(),
            resources: core::default::Default::default(),
            roles: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressToRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressToRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressToRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressToRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressToRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `external_resources` after provisioning.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn external_resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.external_resources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of resources, in the form 'projects/{project_number}',\n'folders/{folder_number}', or 'organizations/{organization_number}',\nthat match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `roles` after provisioning.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'EgressFrom'\nare allowed to perform."]
    pub fn roles(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo_OperationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPoliciesDynamic {
    egress_from: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom>,
    >,
    egress_to: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_from: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_to: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo>>,
    dynamic: AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPoliciesDynamic,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies {
    #[doc =
        "Set the field `title`.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `egress_from`.\n"]
    pub fn set_egress_from(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_from = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_from = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `egress_to`.\n"]
    pub fn set_egress_to(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_to = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_to = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies {
    pub fn build(self) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies {
            title: core::default::Default::default(),
            egress_from: core::default::Default::default(),
            egress_to: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPoliciesRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_from` after provisioning.\n"]
    pub fn egress_from(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFromRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_from", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_to` after provisioning.\n"]
    pub fn egress_to(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressToRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_to", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_SourcesDynamic {
    psc_endpoint: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpoint,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_Sources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_endpoint: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpoint,
        >,
    >,
    dynamic: AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_SourcesDynamic,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_Sources {
    #[doc =
        "Set the field `access_level`.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."]
    pub fn set_access_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_level = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource`.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nProjects, folders, organizations, and VPC networks are allowed.\nProject format: 'projects/{project_number}'\nFolder format: 'folders/{folder_number}'\nOrganization format: 'organizations/{organization_number}'\nVPC network format: '//compute.googleapis.com/projects/{PROJECT_ID}/global/networks/{NAME}'\nThe resource may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."]
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
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpoint,
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

impl terrars
::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_Sources {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_Sources,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_Sources {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_Sources {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_Sources {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_Sources {
            access_level: core::default::Default::default(),
            resource: core::default::Default::default(),
            psc_endpoint: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_SourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_SourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_SourcesRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_SourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_SourcesRef {
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
        "Get a reference to the value of field `resource` after provisioning.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nProjects, folders, organizations, and VPC networks are allowed.\nProject format: 'projects/{project_number}'\nFolder format: 'folders/{folder_number}'\nOrganization format: 'organizations/{organization_number}'\nVPC network format: '//compute.googleapis.com/projects/{PROJECT_ID}/global/networks/{NAME}'\nThe resource may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_endpoint` after provisioning.\n"]
    pub fn psc_endpoint(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources_PscEndpointRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFromDynamic {
    sources: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_Sources,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sources: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_Sources>,
    >,
    dynamic: AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFromDynamic,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom {
    #[doc =
        "Set the field `identities`.\nA list of identities that are allowed access through this ingress policy.\nShould be in the format of email address. The email address should represent\nindividual user or service account only."]
    pub fn set_identities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.identities = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
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
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_Sources,
                        >,
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

impl terrars
::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom {
    type O =
        terrars
        ::BlockAssignable<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom {
    pub fn build(self) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFromRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFromRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFromRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFromRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFromRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identities` after provisioning.\nA list of identities that are allowed access through this ingress policy.\nShould be in the format of email address. The email address should represent\nindividual user or service account only."]
    pub fn identities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.base))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom_SourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations_MethodSelectors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations_MethodSelectors {
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
::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations_MethodSelectors {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations_MethodSelectors,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations_MethodSelectors {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations_MethodSelectors {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations_MethodSelectors {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations_MethodSelectors {
            method: core::default::Default::default(),
            permission: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations_MethodSelectorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations_MethodSelectorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations_MethodSelectorsRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations_MethodSelectorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations_MethodSelectorsRef {
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
struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_OperationsDynamic {
    method_selectors: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations_MethodSelectors,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method_selectors: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations_MethodSelectors,
        >,
    >,
    dynamic: AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_OperationsDynamic,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations {
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
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations_MethodSelectors,
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

impl terrars
::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations {
            service_name: core::default::Default::default(),
            method_selectors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_OperationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_OperationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_OperationsRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_OperationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_OperationsRef {
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
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations_MethodSelectorsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method_selectors", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressToDynamic {
    operations: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    roles: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations>,
    >,
    dynamic: AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressToDynamic,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo {
    #[doc =
        "Set the field `resources`.\nA list of resources protected by this 'ServicePerimeter', in the form\n'projects/{project_number}', 'folders/{folder_number}', or\n'organizations/{organization_number}',\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `roles`.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'IngressFrom'\nare allowed to perform."]
    pub fn set_roles(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
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
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_Operations,
                        >,
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

impl terrars
::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo {
    type O =
        terrars
        ::BlockAssignable<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo {
    pub fn build(self) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo {
            resources: core::default::Default::default(),
            roles: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressToRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressToRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressToRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressToRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressToRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of resources protected by this 'ServicePerimeter', in the form\n'projects/{project_number}', 'folders/{folder_number}', or\n'organizations/{organization_number}',\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `roles` after provisioning.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'IngressFrom'\nare allowed to perform."]
    pub fn roles(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo_OperationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPoliciesDynamic {
    ingress_from: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom>,
    >,
    ingress_to: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_from: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_to: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo>>,
    dynamic: AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPoliciesDynamic,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies {
    #[doc =
        "Set the field `title`.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ingress_from`.\n"]
    pub fn set_ingress_from(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFrom,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress_from = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_from = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ingress_to`.\n"]
    pub fn set_ingress_to(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressTo,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress_to = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_to = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies {
    pub fn build(self) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies {
            title: core::default::Default::default(),
            ingress_from: core::default::Default::default(),
            ingress_to: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPoliciesRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress_from` after provisioning.\n"]
    pub fn ingress_from(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressFromRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_from", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress_to` after provisioning.\n"]
    pub fn ingress_to(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies_IngressToRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_to", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers_AddRequestHeader {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers_AddRequestHeader {

}

impl terrars
::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers_AddRequestHeader {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers_AddRequestHeader,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers_AddRequestHeader {
    #[doc = "HTTP header key."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "HTTP header value."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers_AddRequestHeader {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers_AddRequestHeader {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers_AddRequestHeader {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers_AddRequestHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers_AddRequestHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers_AddRequestHeaderRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers_AddRequestHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers_AddRequestHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nHTTP header key."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nHTTP header value."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_ModifiersDynamic {
    add_request_header: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers_AddRequestHeader,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add_request_header: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers_AddRequestHeader,
        >,
    >,
    dynamic: AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_ModifiersDynamic,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers {
    #[doc = "Set the field `add_request_header`.\n"]
    pub fn set_add_request_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers_AddRequestHeader,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.add_request_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.add_request_header = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers {
            add_request_header: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_ModifiersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_ModifiersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_ModifiersRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_ModifiersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_ModifiersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `add_request_header` after provisioning.\n"]
    pub fn add_request_header(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers_AddRequestHeaderRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.add_request_header", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatternsDynamic {
    modifiers: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pattern: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    modifiers: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers,
        >,
    >,
    dynamic: AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatternsDynamic,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns {
    #[doc = "Set the field `pattern`.\nURL pattern to allow."]
    pub fn set_pattern(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pattern = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\nSupported service to allow."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `modifiers`.\n"]
    pub fn set_modifiers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_Modifiers,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.modifiers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.modifiers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns {
            pattern: core::default::Default::default(),
            service: core::default::Default::default(),
            modifiers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatternsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatternsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatternsRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatternsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatternsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\nURL pattern to allow."]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\nSupported service to allow."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `modifiers` after provisioning.\n"]
    pub fn modifiers(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns_ModifiersRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.modifiers", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServicesDynamic {
    allowed_service_patterns: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_services: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_restriction: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_patterns_enforcement_scopes: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_service_patterns: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns,
        >,
    >,
    dynamic: AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServicesDynamic,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices {
    #[doc =
        "Set the field `allowed_services`.\nThe list of APIs usable within the Service Perimeter.\nMust be empty unless 'enableRestriction' is True."]
    pub fn set_allowed_services(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_services = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_restriction`.\nWhether to restrict API calls within the Service Perimeter to the\nlist of APIs specified in 'allowedServices'."]
    pub fn set_enable_restriction(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_restriction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_patterns_enforcement_scopes`.\nDefines the enforcement scopes of service patterns. Possible values: [\"GOOGLE_APIS_VIA_PRIVATE_PATH\"]"]
    pub fn set_service_patterns_enforcement_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.service_patterns_enforcement_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_service_patterns`.\n"]
    pub fn set_allowed_service_patterns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatterns,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowed_service_patterns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_service_patterns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices {
    type O =
        terrars::BlockAssignable<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices {
    pub fn build(self) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices {
            allowed_services: core::default::Default::default(),
            enable_restriction: core::default::Default::default(),
            service_patterns_enforcement_scopes: core::default::Default::default(),
            allowed_service_patterns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServicesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServicesRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServicesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_services` after provisioning.\nThe list of APIs usable within the Service Perimeter.\nMust be empty unless 'enableRestriction' is True."]
    pub fn allowed_services(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_services", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_restriction` after provisioning.\nWhether to restrict API calls within the Service Perimeter to the\nlist of APIs specified in 'allowedServices'."]
    pub fn enable_restriction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_restriction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_patterns_enforcement_scopes` after provisioning.\nDefines the enforcement scopes of service patterns. Possible values: [\"GOOGLE_APIS_VIA_PRIVATE_PATH\"]"]
    pub fn service_patterns_enforcement_scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_patterns_enforcement_scopes", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_service_patterns` after provisioning.\n"]
    pub fn allowed_service_patterns(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices_AllowedServicePatternsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_service_patterns", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeters_ServicePerimeters_SpecDynamic {
    egress_policies: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies>>,
    ingress_policies: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies>>,
    vpc_accessible_services: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Spec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_levels: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restricted_services: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_policies: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_policies: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_accessible_services: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices>>,
    dynamic: AccessContextManagerServicePerimeters_ServicePerimeters_SpecDynamic,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Spec {
    #[doc =
        "Set the field `access_levels`.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn set_access_levels(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.access_levels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resources`.\nA list of GCP resources that are inside of the service perimeter.\nProjects, folders, and VPC networks are allowed.\nProject format: 'projects/{project_number}'\nFolder format: 'folders/{folder_number}'\nVPC network format: '//compute.googleapis.com/projects/{PROJECT_ID}/global/networks/{NAME}'"]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `restricted_services`.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn set_restricted_services(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.restricted_services = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `egress_policies`.\n"]
    pub fn set_egress_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ingress_policies`.\n"]
    pub fn set_ingress_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_accessible_services`.\n"]
    pub fn set_vpc_accessible_services(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_accessible_services = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_accessible_services = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Spec {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeters_ServicePerimeters_Spec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Spec {
    pub fn build(self) -> AccessContextManagerServicePerimeters_ServicePerimeters_Spec {
        AccessContextManagerServicePerimeters_ServicePerimeters_Spec {
            access_levels: core::default::Default::default(),
            resources: core::default::Default::default(),
            restricted_services: core::default::Default::default(),
            egress_policies: core::default::Default::default(),
            ingress_policies: core::default::Default::default(),
            vpc_accessible_services: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_SpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_SpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_SpecRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_SpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_SpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_levels` after provisioning.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn access_levels(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.access_levels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of GCP resources that are inside of the service perimeter.\nProjects, folders, and VPC networks are allowed.\nProject format: 'projects/{project_number}'\nFolder format: 'folders/{folder_number}'\nVPC network format: '//compute.googleapis.com/projects/{PROJECT_ID}/global/networks/{NAME}'"]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `restricted_services` after provisioning.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn restricted_services(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.restricted_services", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_policies` after provisioning.\n"]
    pub fn egress_policies(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_policies", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress_policies` after provisioning.\n"]
    pub fn ingress_policies(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_policies", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_accessible_services` after provisioning.\n"]
    pub fn vpc_accessible_services(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_accessible_services", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFromDynamic {
    sources: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFrom {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_restriction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sources: std
    ::option
    ::Option<
        std::vec::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources>,
    >,
    dynamic: AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFromDynamic,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFrom {
    #[doc =
        "Set the field `identities`.\nA list of identities that are allowed access through this 'EgressPolicy'.\nShould be in the format of email address. The email address should\nrepresent individual user or service account only."]
    pub fn set_identities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.identities = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_restriction`.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn set_source_restriction(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_restriction = std::option::Option::Some(v.into());
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
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_Sources,
                        >,
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

impl terrars
::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFrom {
    type O =
        terrars
        ::BlockAssignable<AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFrom>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFrom {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFrom {
    pub fn build(self) -> AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFrom {
        AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFrom {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            source_restriction: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFromRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFromRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFromRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFromRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFromRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identities` after provisioning.\nA list of identities that are allowed access through this 'EgressPolicy'.\nShould be in the format of email address. The email address should\nrepresent individual user or service account only."]
    pub fn identities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_restriction` after provisioning.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn source_restriction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_restriction", self.base))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressFrom_SourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPoliciesDynamic {
    egress_from: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFrom>,
    >,
    egress_to: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_from: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFrom>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_to: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo>>,
    dynamic: AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPoliciesDynamic,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies {
    #[doc =
        "Set the field `title`.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `egress_from`.\n"]
    pub fn set_egress_from(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFrom,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_from = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_from = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `egress_to`.\n"]
    pub fn set_egress_to(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressTo,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_to = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_to = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies {
    pub fn build(self) -> AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies {
        AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies {
            title: core::default::Default::default(),
            egress_from: core::default::Default::default(),
            egress_to: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPoliciesRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_from` after provisioning.\n"]
    pub fn egress_from(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies_EgressFromRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_from", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_to` after provisioning.\n"]
    pub fn egress_to(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_EgressPolicies_EgressToRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_to", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeters_ServicePerimeters_StatusDynamic {
    egress_policies: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies>>,
    ingress_policies: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies>>,
    vpc_accessible_services: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_levels: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restricted_services: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_policies: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_policies: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_accessible_services: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices>>,
    dynamic: AccessContextManagerServicePerimeters_ServicePerimeters_StatusDynamic,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_Status {
    #[doc =
        "Set the field `access_levels`.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn set_access_levels(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.access_levels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resources`.\nA list of GCP resources that are inside of the service perimeter.\nProjects, folders, and VPC networks are allowed.\nProject format: 'projects/{project_number}'\nFolder format: 'folders/{folder_number}'\nVPC network format: '//compute.googleapis.com/projects/{PROJECT_ID}/global/networks/{NAME}'"]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `restricted_services`.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn set_restricted_services(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.restricted_services = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `egress_policies`.\n"]
    pub fn set_egress_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPolicies,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ingress_policies`.\n"]
    pub fn set_ingress_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_IngressPolicies,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_accessible_services`.\n"]
    pub fn set_vpc_accessible_services(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServices,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_accessible_services = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_accessible_services = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters_Status {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeters_ServicePerimeters_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters_Status {}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters_Status {
    pub fn build(self) -> AccessContextManagerServicePerimeters_ServicePerimeters_Status {
        AccessContextManagerServicePerimeters_ServicePerimeters_Status {
            access_levels: core::default::Default::default(),
            resources: core::default::Default::default(),
            restricted_services: core::default::Default::default(),
            egress_policies: core::default::Default::default(),
            ingress_policies: core::default::Default::default(),
            vpc_accessible_services: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeters_ServicePerimeters_StatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimeters_StatusRef {
        AccessContextManagerServicePerimeters_ServicePerimeters_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimeters_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_levels` after provisioning.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn access_levels(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.access_levels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of GCP resources that are inside of the service perimeter.\nProjects, folders, and VPC networks are allowed.\nProject format: 'projects/{project_number}'\nFolder format: 'folders/{folder_number}'\nVPC network format: '//compute.googleapis.com/projects/{PROJECT_ID}/global/networks/{NAME}'"]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `restricted_services` after provisioning.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn restricted_services(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.restricted_services", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_policies` after provisioning.\n"]
    pub fn egress_policies(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeters_ServicePerimeters_Status_EgressPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_policies", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_accessible_services` after provisioning.\n"]
    pub fn vpc_accessible_services(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeters_ServicePerimeters_Spec_VpcAccessibleServicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_accessible_services", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeters_ServicePerimetersDynamic {
    spec: std::option::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeters_ServicePerimeters_Spec>>,
    status: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeters_ServicePerimeters_Status>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_ServicePerimeters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    perimeter_type: std::option::Option<terrars::PrimField<std::string::String>>,
    title: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_explicit_dry_run_spec: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Spec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeters_ServicePerimeters_Status>>,
    dynamic: AccessContextManagerServicePerimeters_ServicePerimetersDynamic,
}

impl AccessContextManagerServicePerimeters_ServicePerimeters {
    #[doc =
        "Set the field `description`.\nDescription of the ServicePerimeter and its use. Does not affect\nbehavior."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `perimeter_type`.\nSpecifies the type of the Perimeter. There are two types: regular and\nbridge. Regular Service Perimeter contains resources, access levels,\nand restricted services. Every resource can be in at most\nONE regular Service Perimeter.\n\nIn addition to being in a regular service perimeter, a resource can also\nbe in zero or more perimeter bridges. A perimeter bridge only contains\nresources. Cross project operations are permitted if all effected\nresources share some perimeter (whether bridge or regular). Perimeter\nBridge does not contain access levels or services: those are governed\nentirely by the regular perimeter that resource is in.\n\nPerimeter Bridges are typically useful when building more complex\ntopologies with many independent perimeters that need to share some data\nwith a common perimeter, but should not be able to share data among\nthemselves. Default value: \"PERIMETER_TYPE_REGULAR\" Possible values: [\"PERIMETER_TYPE_REGULAR\", \"PERIMETER_TYPE_BRIDGE\"]"]
    pub fn set_perimeter_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.perimeter_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_explicit_dry_run_spec`.\nUse explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists\nfor all Service Perimeters, and that spec is identical to the status for those\nService Perimeters. When this flag is set, it inhibits the generation of the\nimplicit spec, thereby allowing the user to explicitly provide a\nconfiguration (\"spec\") to use in a dry-run version of the Service Perimeter.\nThis allows the user to test changes to the enforced config (\"status\") without\nactually enforcing them. This testing is done through analyzing the differences\nbetween currently enforced and suggested restrictions. useExplicitDryRunSpec must\nbet set to True if any of the fields in the spec are set to non-default values."]
    pub fn set_use_explicit_dry_run_spec(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_explicit_dry_run_spec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessContextManagerServicePerimeters_ServicePerimeters_Spec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessContextManagerServicePerimeters_ServicePerimeters_Status>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.status = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.status = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeters_ServicePerimeters {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeters_ServicePerimeters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_ServicePerimeters {
    #[doc =
        "Resource name for the ServicePerimeter. The short_name component must\nbegin with a letter and only include alphanumeric and '_'.\nFormat: accessPolicies/{policy_id}/servicePerimeters/{short_name}"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Human readable title. Must be unique within the Policy."]
    pub title: terrars::PrimField<std::string::String>,
}

impl BuildAccessContextManagerServicePerimeters_ServicePerimeters {
    pub fn build(self) -> AccessContextManagerServicePerimeters_ServicePerimeters {
        AccessContextManagerServicePerimeters_ServicePerimeters {
            description: core::default::Default::default(),
            name: self.name,
            perimeter_type: core::default::Default::default(),
            title: self.title,
            use_explicit_dry_run_spec: core::default::Default::default(),
            spec: core::default::Default::default(),
            status: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_ServicePerimetersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeters_ServicePerimetersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_ServicePerimetersRef {
        AccessContextManagerServicePerimeters_ServicePerimetersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_ServicePerimetersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the AccessPolicy was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the ServicePerimeter and its use. Does not affect\nbehavior."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nResource name for the ServicePerimeter. The short_name component must\nbegin with a letter and only include alphanumeric and '_'.\nFormat: accessPolicies/{policy_id}/servicePerimeters/{short_name}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `perimeter_type` after provisioning.\nSpecifies the type of the Perimeter. There are two types: regular and\nbridge. Regular Service Perimeter contains resources, access levels,\nand restricted services. Every resource can be in at most\nONE regular Service Perimeter.\n\nIn addition to being in a regular service perimeter, a resource can also\nbe in zero or more perimeter bridges. A perimeter bridge only contains\nresources. Cross project operations are permitted if all effected\nresources share some perimeter (whether bridge or regular). Perimeter\nBridge does not contain access levels or services: those are governed\nentirely by the regular perimeter that resource is in.\n\nPerimeter Bridges are typically useful when building more complex\ntopologies with many independent perimeters that need to share some data\nwith a common perimeter, but should not be able to share data among\nthemselves. Default value: \"PERIMETER_TYPE_REGULAR\" Possible values: [\"PERIMETER_TYPE_REGULAR\", \"PERIMETER_TYPE_BRIDGE\"]"]
    pub fn perimeter_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.perimeter_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the Policy."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the AccessPolicy was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_explicit_dry_run_spec` after provisioning.\nUse explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists\nfor all Service Perimeters, and that spec is identical to the status for those\nService Perimeters. When this flag is set, it inhibits the generation of the\nimplicit spec, thereby allowing the user to explicitly provide a\nconfiguration (\"spec\") to use in a dry-run version of the Service Perimeter.\nThis allows the user to test changes to the enforced config (\"status\") without\nactually enforcing them. This testing is done through analyzing the differences\nbetween currently enforced and suggested restrictions. useExplicitDryRunSpec must\nbet set to True if any of the fields in the spec are set to non-default values."]
    pub fn use_explicit_dry_run_spec(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_explicit_dry_run_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<AccessContextManagerServicePerimeters_ServicePerimeters_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::ListRef<AccessContextManagerServicePerimeters_ServicePerimeters_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeters_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeters_Timeouts {
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

impl terrars::ToListMappable for AccessContextManagerServicePerimeters_Timeouts {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeters_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeters_Timeouts {}

impl BuildAccessContextManagerServicePerimeters_Timeouts {
    pub fn build(self) -> AccessContextManagerServicePerimeters_Timeouts {
        AccessContextManagerServicePerimeters_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerServicePerimeters_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeters_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeters_TimeoutsRef {
        AccessContextManagerServicePerimeters_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeters_TimeoutsRef {
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
