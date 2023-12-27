use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GkeHubScopeRbacRoleBindingData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    scope_id: PrimField<String>,
    scope_rbac_role_binding_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<Vec<GkeHubScopeRbacRoleBindingRoleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GkeHubScopeRbacRoleBindingTimeoutsEl>,
    dynamic: GkeHubScopeRbacRoleBindingDynamic,
}

struct GkeHubScopeRbacRoleBinding_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GkeHubScopeRbacRoleBindingData>,
}

#[derive(Clone)]
pub struct GkeHubScopeRbacRoleBinding(Rc<GkeHubScopeRbacRoleBinding_>);

impl GkeHubScopeRbacRoleBinding {
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

    #[doc= "Set the field `group`.\nPrincipal that is be authorized in the cluster (at least of one the oneof\nis required). Updating one will unset the other automatically.\ngroup is the group, as seen by the kubernetes cluster."]
    pub fn set_group(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().group = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nLabels for this ScopeRBACRoleBinding.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `user`.\nPrincipal that is be authorized in the cluster (at least of one the oneof\nis required). Updating one will unset the other automatically.\nuser is the name of the user as seen by the kubernetes cluster, example\n\"alice\" or \"alice@domain.tld\""]
    pub fn set_user(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user = Some(v.into());
        self
    }

    #[doc= "Set the field `role`.\n"]
    pub fn set_role(self, v: impl Into<BlockAssignable<GkeHubScopeRbacRoleBindingRoleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().role = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.role = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GkeHubScopeRbacRoleBindingTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nTime the RBAC Role Binding was created in UTC."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_time` after provisioning.\nTime the RBAC Role Binding was deleted in UTC."]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\nPrincipal that is be authorized in the cluster (at least of one the oneof\nis required). Updating one will unset the other automatically.\ngroup is the group, as seen by the kubernetes cluster."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nLabels for this ScopeRBACRoleBinding.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name for the RBAC Role Binding"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scope_id` after provisioning.\nId of the scope"]
    pub fn scope_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scope_rbac_role_binding_id` after provisioning.\nThe client-provided identifier of the RBAC Role Binding."]
    pub fn scope_rbac_role_binding_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope_rbac_role_binding_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nState of the RBAC Role Binding resource."]
    pub fn state(&self) -> ListRef<GkeHubScopeRbacRoleBindingStateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uid` after provisioning.\nGoogle-generated UUID for this resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_time` after provisioning.\nTime the RBAC Role Binding was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user` after provisioning.\nPrincipal that is be authorized in the cluster (at least of one the oneof\nis required). Updating one will unset the other automatically.\nuser is the name of the user as seen by the kubernetes cluster, example\n\"alice\" or \"alice@domain.tld\""]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> ListRef<GkeHubScopeRbacRoleBindingRoleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubScopeRbacRoleBindingTimeoutsElRef {
        GkeHubScopeRbacRoleBindingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for GkeHubScopeRbacRoleBinding {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GkeHubScopeRbacRoleBinding { }

impl ToListMappable for GkeHubScopeRbacRoleBinding {
    type O = ListRef<GkeHubScopeRbacRoleBindingRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GkeHubScopeRbacRoleBinding_ {
    fn extract_resource_type(&self) -> String {
        "google_gke_hub_scope_rbac_role_binding".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGkeHubScopeRbacRoleBinding {
    pub tf_id: String,
    #[doc= "Id of the scope"]
    pub scope_id: PrimField<String>,
    #[doc= "The client-provided identifier of the RBAC Role Binding."]
    pub scope_rbac_role_binding_id: PrimField<String>,
}

impl BuildGkeHubScopeRbacRoleBinding {
    pub fn build(self, stack: &mut Stack) -> GkeHubScopeRbacRoleBinding {
        let out = GkeHubScopeRbacRoleBinding(Rc::new(GkeHubScopeRbacRoleBinding_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GkeHubScopeRbacRoleBindingData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                group: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                project: core::default::Default::default(),
                scope_id: self.scope_id,
                scope_rbac_role_binding_id: self.scope_rbac_role_binding_id,
                user: core::default::Default::default(),
                role: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GkeHubScopeRbacRoleBindingRef {
    shared: StackShared,
    base: String,
}

impl Ref for GkeHubScopeRbacRoleBindingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GkeHubScopeRbacRoleBindingRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nTime the RBAC Role Binding was created in UTC."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_time` after provisioning.\nTime the RBAC Role Binding was deleted in UTC."]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\nPrincipal that is be authorized in the cluster (at least of one the oneof\nis required). Updating one will unset the other automatically.\ngroup is the group, as seen by the kubernetes cluster."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nLabels for this ScopeRBACRoleBinding.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name for the RBAC Role Binding"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scope_id` after provisioning.\nId of the scope"]
    pub fn scope_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scope_rbac_role_binding_id` after provisioning.\nThe client-provided identifier of the RBAC Role Binding."]
    pub fn scope_rbac_role_binding_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope_rbac_role_binding_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nState of the RBAC Role Binding resource."]
    pub fn state(&self) -> ListRef<GkeHubScopeRbacRoleBindingStateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uid` after provisioning.\nGoogle-generated UUID for this resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_time` after provisioning.\nTime the RBAC Role Binding was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user` after provisioning.\nPrincipal that is be authorized in the cluster (at least of one the oneof\nis required). Updating one will unset the other automatically.\nuser is the name of the user as seen by the kubernetes cluster, example\n\"alice\" or \"alice@domain.tld\""]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> ListRef<GkeHubScopeRbacRoleBindingRoleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubScopeRbacRoleBindingTimeoutsElRef {
        GkeHubScopeRbacRoleBindingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct GkeHubScopeRbacRoleBindingStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
}

impl GkeHubScopeRbacRoleBindingStateEl {
    #[doc= "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.code = Some(v.into());
        self
    }
}

impl ToListMappable for GkeHubScopeRbacRoleBindingStateEl {
    type O = BlockAssignable<GkeHubScopeRbacRoleBindingStateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubScopeRbacRoleBindingStateEl {}

impl BuildGkeHubScopeRbacRoleBindingStateEl {
    pub fn build(self) -> GkeHubScopeRbacRoleBindingStateEl {
        GkeHubScopeRbacRoleBindingStateEl { code: core::default::Default::default() }
    }
}

pub struct GkeHubScopeRbacRoleBindingStateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GkeHubScopeRbacRoleBindingStateElRef {
    fn new(shared: StackShared, base: String) -> GkeHubScopeRbacRoleBindingStateElRef {
        GkeHubScopeRbacRoleBindingStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubScopeRbacRoleBindingStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubScopeRbacRoleBindingRoleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    predefined_role: Option<PrimField<String>>,
}

impl GkeHubScopeRbacRoleBindingRoleEl {
    #[doc= "Set the field `predefined_role`.\nPredefinedRole is an ENUM representation of the default Kubernetes Roles Possible values: [\"UNKNOWN\", \"ADMIN\", \"EDIT\", \"VIEW\"]"]
    pub fn set_predefined_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.predefined_role = Some(v.into());
        self
    }
}

impl ToListMappable for GkeHubScopeRbacRoleBindingRoleEl {
    type O = BlockAssignable<GkeHubScopeRbacRoleBindingRoleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubScopeRbacRoleBindingRoleEl {}

impl BuildGkeHubScopeRbacRoleBindingRoleEl {
    pub fn build(self) -> GkeHubScopeRbacRoleBindingRoleEl {
        GkeHubScopeRbacRoleBindingRoleEl { predefined_role: core::default::Default::default() }
    }
}

pub struct GkeHubScopeRbacRoleBindingRoleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GkeHubScopeRbacRoleBindingRoleElRef {
    fn new(shared: StackShared, base: String) -> GkeHubScopeRbacRoleBindingRoleElRef {
        GkeHubScopeRbacRoleBindingRoleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubScopeRbacRoleBindingRoleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `predefined_role` after provisioning.\nPredefinedRole is an ENUM representation of the default Kubernetes Roles Possible values: [\"UNKNOWN\", \"ADMIN\", \"EDIT\", \"VIEW\"]"]
    pub fn predefined_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.predefined_role", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubScopeRbacRoleBindingTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl GkeHubScopeRbacRoleBindingTimeoutsEl {
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

impl ToListMappable for GkeHubScopeRbacRoleBindingTimeoutsEl {
    type O = BlockAssignable<GkeHubScopeRbacRoleBindingTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubScopeRbacRoleBindingTimeoutsEl {}

impl BuildGkeHubScopeRbacRoleBindingTimeoutsEl {
    pub fn build(self) -> GkeHubScopeRbacRoleBindingTimeoutsEl {
        GkeHubScopeRbacRoleBindingTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct GkeHubScopeRbacRoleBindingTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GkeHubScopeRbacRoleBindingTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GkeHubScopeRbacRoleBindingTimeoutsElRef {
        GkeHubScopeRbacRoleBindingTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubScopeRbacRoleBindingTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct GkeHubScopeRbacRoleBindingDynamic {
    role: Option<DynamicBlock<GkeHubScopeRbacRoleBindingRoleEl>>,
}
