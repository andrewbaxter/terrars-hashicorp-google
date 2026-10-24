use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GkeHubFeatureData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleet_default_member_config: std::option::Option<std::vec::Vec<GkeHubFeatureFleetDefaultMemberConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<GkeHubFeatureSpecEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GkeHubFeatureTimeoutsEl>,
    dynamic: GkeHubFeatureDynamic,
}

struct GkeHubFeature_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GkeHubFeatureData>,
}

#[derive(Clone)]
pub struct GkeHubFeature(std::rc::Rc<GkeHubFeature_>);

impl GkeHubFeature {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nGCP labels for this Feature.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe full, unique name of this Feature resource"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fleet_default_member_config`.\n"]
    pub fn set_fleet_default_member_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fleet_default_member_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fleet_default_member_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<GkeHubFeatureSpecEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GkeHubFeatureTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. When the Feature resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. When the Feature resource was deleted."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nGCP labels for this Feature.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full, unique name of this Feature resource"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_state` after provisioning.\nState of the Feature resource itself."]
    pub fn resource_state(&self) -> terrars::ListRef<GkeHubFeatureResourceStateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The Hub-wide Feature state"]
    pub fn state(&self) -> terrars::ListRef<GkeHubFeatureStateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. When the Feature resource was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet_default_member_config` after provisioning.\n"]
    pub fn fleet_default_member_config(&self) -> terrars::ListRef<GkeHubFeatureFleetDefaultMemberConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet_default_member_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<GkeHubFeatureSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubFeatureTimeoutsElRef {
        GkeHubFeatureTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for GkeHubFeature {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GkeHubFeature { }

impl terrars::ToListMappable for GkeHubFeature {
    type O = terrars::ListRef<GkeHubFeatureRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GkeHubFeature_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_gke_hub_feature".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGkeHubFeature {
    pub tf_id: std::string::String,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildGkeHubFeature {
    pub fn build(self, stack: &mut terrars::Stack) -> GkeHubFeature {
        let out = GkeHubFeature(std::rc::Rc::new(GkeHubFeature_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GkeHubFeatureData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                fleet_default_member_config: core::default::Default::default(),
                spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GkeHubFeatureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GkeHubFeatureRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. When the Feature resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. When the Feature resource was deleted."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nGCP labels for this Feature.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full, unique name of this Feature resource"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_state` after provisioning.\nState of the Feature resource itself."]
    pub fn resource_state(&self) -> terrars::ListRef<GkeHubFeatureResourceStateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The Hub-wide Feature state"]
    pub fn state(&self) -> terrars::ListRef<GkeHubFeatureStateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. When the Feature resource was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet_default_member_config` after provisioning.\n"]
    pub fn fleet_default_member_config(&self) -> terrars::ListRef<GkeHubFeatureFleetDefaultMemberConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet_default_member_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<GkeHubFeatureSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubFeatureTimeoutsElRef {
        GkeHubFeatureTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureResourceStateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    has_resources: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureResourceStateEl {
    #[doc = "Set the field `has_resources`.\n"]
    pub fn set_has_resources(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.has_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureResourceStateEl {
    type O = terrars::BlockAssignable<GkeHubFeatureResourceStateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureResourceStateEl {}

impl BuildGkeHubFeatureResourceStateEl {
    pub fn build(self) -> GkeHubFeatureResourceStateEl {
        GkeHubFeatureResourceStateEl {
            has_resources: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureResourceStateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureResourceStateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubFeatureResourceStateElRef {
        GkeHubFeatureResourceStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureResourceStateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `has_resources` after provisioning.\n"]
    pub fn has_resources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_resources", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureStateElStateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureStateElStateEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureStateElStateEl {
    type O = terrars::BlockAssignable<GkeHubFeatureStateElStateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureStateElStateEl {}

impl BuildGkeHubFeatureStateElStateEl {
    pub fn build(self) -> GkeHubFeatureStateElStateEl {
        GkeHubFeatureStateElStateEl {
            code: core::default::Default::default(),
            description: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureStateElStateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureStateElStateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubFeatureStateElStateElRef {
        GkeHubFeatureStateElStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureStateElStateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureStateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::ListField<GkeHubFeatureStateElStateEl>>,
}

impl GkeHubFeatureStateEl {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::ListField<GkeHubFeatureStateElStateEl>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureStateEl {
    type O = terrars::BlockAssignable<GkeHubFeatureStateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureStateEl {}

impl BuildGkeHubFeatureStateEl {
    pub fn build(self) -> GkeHubFeatureStateEl {
        GkeHubFeatureStateEl { state: core::default::Default::default() }
    }
}

pub struct GkeHubFeatureStateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureStateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubFeatureStateElRef {
        GkeHubFeatureStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureStateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::ListRef<GkeHubFeatureStateElStateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    https_proxy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    secret_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_repo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_rev: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_wait_secs: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
    #[doc =
        "Set the field `gcp_service_account_email`.\nThe Google Cloud Service Account Email used for auth when secretType is gcpServiceAccount"]
    pub fn set_gcp_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `https_proxy`.\nURL for the HTTPS Proxy to be used when communicating with the Git repo"]
    pub fn set_https_proxy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.https_proxy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `policy_dir`.\nThe path within the Git repository that represents the top level of the repo to sync"]
    pub fn set_policy_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_branch`.\nThe branch of the repository to sync from. Default: master"]
    pub fn set_sync_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_repo`.\nThe URL of the Git repository to use as the source of truth"]
    pub fn set_sync_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_repo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_rev`.\nGit revision (tag or hash) to check out. Default HEAD"]
    pub fn set_sync_rev(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_rev = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_wait_secs`.\nPeriod in seconds between consecutive syncs. Default: 15"]
    pub fn set_sync_wait_secs(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_wait_secs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
    type O = terrars::BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
    #[doc = "Type of secret configured for access to the Git repo"]
    pub secret_type: terrars::PrimField<std::string::String>,
}

impl BuildGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
    pub fn build(self) -> GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
        GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
            gcp_service_account_email: core::default::Default::default(),
            https_proxy: core::default::Default::default(),
            policy_dir: core::default::Default::default(),
            secret_type: self.secret_type,
            sync_branch: core::default::Default::default(),
            sync_repo: core::default::Default::default(),
            sync_rev: core::default::Default::default(),
            sync_wait_secs: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitElRef {
        GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gcp_service_account_email` after provisioning.\nThe Google Cloud Service Account Email used for auth when secretType is gcpServiceAccount"]
    pub fn gcp_service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_service_account_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `https_proxy` after provisioning.\nURL for the HTTPS Proxy to be used when communicating with the Git repo"]
    pub fn https_proxy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_proxy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `policy_dir` after provisioning.\nThe path within the Git repository that represents the top level of the repo to sync"]
    pub fn policy_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_dir", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_type` after provisioning.\nType of secret configured for access to the Git repo"]
    pub fn secret_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sync_branch` after provisioning.\nThe branch of the repository to sync from. Default: master"]
    pub fn sync_branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sync_repo` after provisioning.\nThe URL of the Git repository to use as the source of truth"]
    pub fn sync_repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_repo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sync_rev` after provisioning.\nGit revision (tag or hash) to check out. Default HEAD"]
    pub fn sync_rev(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_rev", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sync_wait_secs` after provisioning.\nPeriod in seconds between consecutive syncs. Default: 15"]
    pub fn sync_wait_secs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_wait_secs", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    secret_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_repo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_wait_secs: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
    #[doc =
        "Set the field `gcp_service_account_email`.\nThe Google Cloud Service Account Email used for auth when secretType is gcpServiceAccount"]
    pub fn set_gcp_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `policy_dir`.\nThe absolute path of the directory that contains the local resources. Default: the root directory of the image"]
    pub fn set_policy_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_repo`.\nThe OCI image repository URL for the package to sync from"]
    pub fn set_sync_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_repo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_wait_secs`.\nPeriod in seconds between consecutive syncs. Default: 15"]
    pub fn set_sync_wait_secs(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_wait_secs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nVersion of ACM installed"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
    type O = terrars::BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
    #[doc = "Type of secret configured for access to the Git repo"]
    pub secret_type: terrars::PrimField<std::string::String>,
}

impl BuildGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
    pub fn build(self) -> GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
        GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
            gcp_service_account_email: core::default::Default::default(),
            policy_dir: core::default::Default::default(),
            secret_type: self.secret_type,
            sync_repo: core::default::Default::default(),
            sync_wait_secs: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciElRef {
        GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gcp_service_account_email` after provisioning.\nThe Google Cloud Service Account Email used for auth when secretType is gcpServiceAccount"]
    pub fn gcp_service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_service_account_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `policy_dir` after provisioning.\nThe absolute path of the directory that contains the local resources. Default: the root directory of the image"]
    pub fn policy_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_dir", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_type` after provisioning.\nType of secret configured for access to the Git repo"]
    pub fn secret_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sync_repo` after provisioning.\nThe OCI image repository URL for the package to sync from"]
    pub fn sync_repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_repo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sync_wait_secs` after provisioning.\nPeriod in seconds between consecutive syncs. Default: 15"]
    pub fn sync_wait_secs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_wait_secs", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of ACM installed"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElDynamic {
    git: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl>>,
    oci: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl>>,
}

#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<std::vec::Vec<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oci: std::option::Option<std::vec::Vec<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl>>,
    dynamic: GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElDynamic,
}

impl GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {
    #[doc =
        "Set the field `source_format`.\nSpecifies whether the Config Sync Repo is in hierarchical or unstructured mode"]
    pub fn set_source_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.git = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.git = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oci`.\n"]
    pub fn set_oci(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oci = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oci = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {
    type O = terrars::BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {}

impl BuildGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {
    pub fn build(self) -> GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {
        GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {
            source_format: core::default::Default::default(),
            git: core::default::Default::default(),
            oci: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElRef {
        GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `source_format` after provisioning.\nSpecifies whether the Config Sync Repo is in hierarchical or unstructured mode"]
    pub fn source_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_format", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `oci` after provisioning.\n"]
    pub fn oci(&self) -> terrars::ListRef<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oci", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElDynamic {
    config_sync: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl>>,
}

#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_sync: std
    ::option
    ::Option<std::vec::Vec<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl>>,
    dynamic: GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElDynamic,
}

impl GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {
    #[doc = "Set the field `config_sync`.\n"]
    pub fn set_config_sync(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.config_sync = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.config_sync = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {
    type O = terrars::BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {}

impl BuildGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {
    pub fn build(self) -> GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {
        GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {
            config_sync: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElRef {
        GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `config_sync` after provisioning.\n"]
    pub fn config_sync(
        &self,
    ) -> terrars::ListRef<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_sync", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElMeshEl {
    management: terrars::PrimField<std::string::String>,
}

impl GkeHubFeatureFleetDefaultMemberConfigElMeshEl { }

impl terrars::ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElMeshEl {
    type O = terrars::BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElMeshEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElMeshEl {
    #[doc =
        "Whether to automatically manage Service Mesh Possible values: [\"MANAGEMENT_UNSPECIFIED\", \"MANAGEMENT_AUTOMATIC\", \"MANAGEMENT_MANUAL\"]"]
    pub management: terrars::PrimField<std::string::String>,
}

impl BuildGkeHubFeatureFleetDefaultMemberConfigElMeshEl {
    pub fn build(self) -> GkeHubFeatureFleetDefaultMemberConfigElMeshEl {
        GkeHubFeatureFleetDefaultMemberConfigElMeshEl { management: self.management }
    }
}

pub struct GkeHubFeatureFleetDefaultMemberConfigElMeshElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureFleetDefaultMemberConfigElMeshElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElMeshElRef {
        GkeHubFeatureFleetDefaultMemberConfigElMeshElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureFleetDefaultMemberConfigElMeshElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `management` after provisioning.\nWhether to automatically manage Service Mesh Possible values: [\"MANAGEMENT_UNSPECIFIED\", \"MANAGEMENT_AUTOMATIC\", \"MANAGEMENT_MANUAL\"]"]
    pub fn management(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.management", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl {
    #[doc = "Set the field `cpu`.\nCPU requirement expressed in Kubernetes resource units."]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\nMemory requirement expressed in Kubernetes resource units."]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl {}

impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl {
    pub fn build(
        self,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl {
            cpu: core::default::Default::default(),
            memory: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsElRef {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpu` after provisioning.\nCPU requirement expressed in Kubernetes resource units."]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory` after provisioning.\nMemory requirement expressed in Kubernetes resource units."]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl {
    #[doc = "Set the field `cpu`.\nCPU requirement expressed in Kubernetes resource units."]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\nMemory requirement expressed in Kubernetes resource units."]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl {}

impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl {
    pub fn build(
        self,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl {
            cpu: core::default::Default::default(),
            memory: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsElRef {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpu` after provisioning.\nCPU requirement expressed in Kubernetes resource units."]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory` after provisioning.\nMemory requirement expressed in Kubernetes resource units."]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElDynamic {
    limits: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl,
        >,
    >,
    requests: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requests: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl,
        >,
    >,
    dynamic: GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElDynamic,
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl {
    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.limits = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.limits = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `requests`.\n"]
    pub fn set_requests(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.requests = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.requests = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl {}

impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl {
    pub fn build(
        self,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl {
            limits: core::default::Default::default(),
            requests: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRef {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(
        &self,
    ) -> terrars
    ::ListRef<
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc = "Get a reference to the value of field `requests` after provisioning.\n"]
    pub fn requests(
        &self,
    ) -> terrars
    ::ListRef<
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requests", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl {
    #[doc = "Set the field `effect`.\nMatches a taint effect."]
    pub fn set_effect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.effect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\nMatches a taint key (not necessarily unique)."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operator`.\nMatches a taint operator."]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nMatches a taint value."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl {}

impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl {
    pub fn build(
        self,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationElRef {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `effect` after provisioning.\nMatches a taint effect."]
    pub fn effect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nMatches a taint key (not necessarily unique)."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\nMatches a taint operator."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nMatches a taint value."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElDynamic {
    container_resources: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl,
        >,
    >,
    pod_toleration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl {
    component: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_affinity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_resources: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_toleration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl,
        >,
    >,
    dynamic: GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElDynamic,
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl {
    #[doc =
        "Set the field `pod_affinity`.\nPod affinity configuration. Possible values: [\"AFFINITY_UNSPECIFIED\", \"NO_AFFINITY\", \"ANTI_AFFINITY\"]"]
    pub fn set_pod_affinity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pod_affinity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replica_count`.\nPod replica count."]
    pub fn set_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.replica_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_resources`.\n"]
    pub fn set_container_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container_resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pod_toleration`.\n"]
    pub fn set_pod_toleration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pod_toleration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pod_toleration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl {
    #[doc = ""]
    pub component: terrars::PrimField<std::string::String>,
}

impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl {
    pub fn build(
        self,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl {
            component: self.component,
            pod_affinity: core::default::Default::default(),
            replica_count: core::default::Default::default(),
            container_resources: core::default::Default::default(),
            pod_toleration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElRef {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.component", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pod_affinity` after provisioning.\nPod affinity configuration. Possible values: [\"AFFINITY_UNSPECIFIED\", \"NO_AFFINITY\", \"ANTI_AFFINITY\"]"]
    pub fn pod_affinity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pod_affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `replica_count` after provisioning.\nPod replica count."]
    pub fn replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_count", self.base))
    }

    #[doc = "Get a reference to the value of field `container_resources` after provisioning.\n"]
    pub fn container_resources(
        &self,
    ) -> terrars
    ::ListRef<
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_resources", self.base))
    }

    #[doc = "Get a reference to the value of field `pod_toleration` after provisioning.\n"]
    pub fn pod_toleration(
        &self,
    ) -> terrars
    ::ListRef<
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_toleration", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backends: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl {
    #[doc =
        "Set the field `backends`.\nSpecifies the list of backends Policy Controller will export to. An empty list would effectively disable metrics export. Possible values: [\"MONITORING_BACKEND_UNSPECIFIED\", \"PROMETHEUS\", \"CLOUD_MONITORING\"]"]
    pub fn set_backends(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.backends = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl {}

impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl {
    pub fn build(
        self,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl {
            backends: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringElRef {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backends` after provisioning.\nSpecifies the list of backends Policy Controller will export to. An empty list would effectively disable metrics export. Possible values: [\"MONITORING_BACKEND_UNSPECIFIED\", \"PROMETHEUS\", \"CLOUD_MONITORING\"]"]
    pub fn backends(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backends", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl {
    bundle: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exempted_namespaces: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl {
    #[doc = "Set the field `exempted_namespaces`.\nThe set of namespaces to be exempted from the bundle."]
    pub fn set_exempted_namespaces(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exempted_namespaces = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl {
    #[doc = ""]
    pub bundle: terrars::PrimField<std::string::String>,
}

impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl {
    pub fn build(
        self,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl {
            bundle: self.bundle,
            exempted_namespaces: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesElRef {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bundle` after provisioning.\n"]
    pub fn bundle(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exempted_namespaces` after provisioning.\nThe set of namespaces to be exempted from the bundle."]
    pub fn exempted_namespaces(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exempted_namespaces", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    installation: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl {
    #[doc =
        "Set the field `installation`.\nConfigures the manner in which the template library is installed on the cluster. Possible values: [\"INSTALATION_UNSPECIFIED\", \"NOT_INSTALLED\", \"ALL\"]"]
    pub fn set_installation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.installation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl {}

impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl {
    pub fn build(
        self,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl {
            installation: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryElRef {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `installation` after provisioning.\nConfigures the manner in which the template library is installed on the cluster. Possible values: [\"INSTALATION_UNSPECIFIED\", \"NOT_INSTALLED\", \"ALL\"]"]
    pub fn installation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.installation", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElDynamic {
    bundles: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl,
        >,
    >,
    template_library: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bundles: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_library: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl,
        >,
    >,
    dynamic: GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElDynamic,
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl {
    #[doc = "Set the field `bundles`.\n"]
    pub fn set_bundles(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bundles = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bundles = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `template_library`.\n"]
    pub fn set_template_library(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.template_library = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.template_library = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl {
    type O =
        terrars
        ::BlockAssignable<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl {}

impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl {
    pub fn build(
        self,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl {
            bundles: core::default::Default::default(),
            template_library: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElRef {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `template_library` after provisioning.\n"]
    pub fn template_library(
        &self,
    ) -> terrars
    ::ListRef<
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template_library", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDynamic {
    deployment_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl,
        >,
    >,
    monitoring: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl,
        >,
    >,
    policy_content: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit_interval_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constraint_violation_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exemptable_namespaces: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    install_spec: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_denies_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mutation_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    referential_rules_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_configs: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_content: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl>,
    >,
    dynamic: GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDynamic,
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl {
    #[doc =
        "Set the field `audit_interval_seconds`.\nInterval for Policy Controller Audit scans (in seconds). When set to 0, this disables audit functionality altogether."]
    pub fn set_audit_interval_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.audit_interval_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `constraint_violation_limit`.\nThe maximum number of audit violations to be stored in a constraint. If not set, the internal default of 20 will be used."]
    pub fn set_constraint_violation_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.constraint_violation_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exemptable_namespaces`.\nThe set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster."]
    pub fn set_exemptable_namespaces(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exemptable_namespaces = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_denies_enabled`.\nLogs all denies and dry run failures."]
    pub fn set_log_denies_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.log_denies_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mutation_enabled`.\nEnables the ability to mutate resources using Policy Controller."]
    pub fn set_mutation_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.mutation_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `referential_rules_enabled`.\nEnables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated."]
    pub fn set_referential_rules_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.referential_rules_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_configs`.\n"]
    pub fn set_deployment_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deployment_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deployment_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monitoring`.\n"]
    pub fn set_monitoring(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monitoring = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monitoring = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `policy_content`.\n"]
    pub fn set_policy_content(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.policy_content = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_content = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl {
    type O =
        terrars
        ::BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl {
    #[doc =
        "Configures the mode of the Policy Controller installation Possible values: [\"INSTALL_SPEC_UNSPECIFIED\", \"INSTALL_SPEC_NOT_INSTALLED\", \"INSTALL_SPEC_ENABLED\", \"INSTALL_SPEC_SUSPENDED\", \"INSTALL_SPEC_DETACHED\"]"]
    pub install_spec: terrars::PrimField<std::string::String>,
}

impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl {
    pub fn build(self) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl {
            audit_interval_seconds: core::default::Default::default(),
            constraint_violation_limit: core::default::Default::default(),
            exemptable_namespaces: core::default::Default::default(),
            install_spec: self.install_spec,
            log_denies_enabled: core::default::Default::default(),
            mutation_enabled: core::default::Default::default(),
            referential_rules_enabled: core::default::Default::default(),
            deployment_configs: core::default::Default::default(),
            monitoring: core::default::Default::default(),
            policy_content: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElRef {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `audit_interval_seconds` after provisioning.\nInterval for Policy Controller Audit scans (in seconds). When set to 0, this disables audit functionality altogether."]
    pub fn audit_interval_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audit_interval_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `constraint_violation_limit` after provisioning.\nThe maximum number of audit violations to be stored in a constraint. If not set, the internal default of 20 will be used."]
    pub fn constraint_violation_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.constraint_violation_limit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exemptable_namespaces` after provisioning.\nThe set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster."]
    pub fn exemptable_namespaces(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exemptable_namespaces", self.base))
    }

    #[doc =
        "Get a reference to the value of field `install_spec` after provisioning.\nConfigures the mode of the Policy Controller installation Possible values: [\"INSTALL_SPEC_UNSPECIFIED\", \"INSTALL_SPEC_NOT_INSTALLED\", \"INSTALL_SPEC_ENABLED\", \"INSTALL_SPEC_SUSPENDED\", \"INSTALL_SPEC_DETACHED\"]"]
    pub fn install_spec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.install_spec", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_denies_enabled` after provisioning.\nLogs all denies and dry run failures."]
    pub fn log_denies_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_denies_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mutation_enabled` after provisioning.\nEnables the ability to mutate resources using Policy Controller."]
    pub fn mutation_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mutation_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `referential_rules_enabled` after provisioning.\nEnables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated."]
    pub fn referential_rules_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.referential_rules_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(
        &self,
    ) -> terrars
    ::ListRef<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_content` after provisioning.\n"]
    pub fn policy_content(
        &self,
    ) -> terrars
    ::ListRef<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_content", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElDynamic {
    policy_controller_hub_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_controller_hub_config: std
    ::option
    ::Option<std::vec::Vec<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl>>,
    dynamic: GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElDynamic,
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {
    #[doc = "Set the field `version`.\nConfigures the version of Policy Controller"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_controller_hub_config`.\n"]
    pub fn set_policy_controller_hub_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.policy_controller_hub_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_controller_hub_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {
    type O = terrars::BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {}

impl BuildGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {
    pub fn build(self) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {
            version: core::default::Default::default(),
            policy_controller_hub_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElRef {
        GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nConfigures the version of Policy Controller"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_controller_hub_config` after provisioning.\n"]
    pub fn policy_controller_hub_config(
        &self,
    ) -> terrars::ListRef<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_controller_hub_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureFleetDefaultMemberConfigElDynamic {
    configmanagement: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl>>,
    mesh: std::option::Option<terrars::DynamicBlock<GkeHubFeatureFleetDefaultMemberConfigElMeshEl>>,
    policycontroller: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl>>,
}

#[derive(Serialize)]
pub struct GkeHubFeatureFleetDefaultMemberConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configmanagement: std::option::Option<std::vec::Vec<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mesh: std::option::Option<std::vec::Vec<GkeHubFeatureFleetDefaultMemberConfigElMeshEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policycontroller: std::option::Option<std::vec::Vec<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl>>,
    dynamic: GkeHubFeatureFleetDefaultMemberConfigElDynamic,
}

impl GkeHubFeatureFleetDefaultMemberConfigEl {
    #[doc = "Set the field `configmanagement`.\n"]
    pub fn set_configmanagement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.configmanagement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.configmanagement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mesh`.\n"]
    pub fn set_mesh(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElMeshEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mesh = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mesh = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `policycontroller`.\n"]
    pub fn set_policycontroller(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.policycontroller = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.policycontroller = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureFleetDefaultMemberConfigEl {
    type O = terrars::BlockAssignable<GkeHubFeatureFleetDefaultMemberConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureFleetDefaultMemberConfigEl {}

impl BuildGkeHubFeatureFleetDefaultMemberConfigEl {
    pub fn build(self) -> GkeHubFeatureFleetDefaultMemberConfigEl {
        GkeHubFeatureFleetDefaultMemberConfigEl {
            configmanagement: core::default::Default::default(),
            mesh: core::default::Default::default(),
            policycontroller: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureFleetDefaultMemberConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureFleetDefaultMemberConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubFeatureFleetDefaultMemberConfigElRef {
        GkeHubFeatureFleetDefaultMemberConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureFleetDefaultMemberConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configmanagement` after provisioning.\n"]
    pub fn configmanagement(&self) -> terrars::ListRef<GkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configmanagement", self.base))
    }

    #[doc = "Get a reference to the value of field `mesh` after provisioning.\n"]
    pub fn mesh(&self) -> terrars::ListRef<GkeHubFeatureFleetDefaultMemberConfigElMeshElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mesh", self.base))
    }

    #[doc = "Get a reference to the value of field `policycontroller` after provisioning.\n"]
    pub fn policycontroller(&self) -> terrars::ListRef<GkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policycontroller", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {
    #[doc =
        "Set the field `mode`.\nSpecified if fleet logging feature is enabled. Possible values: [\"MODE_UNSPECIFIED\", \"COPY\", \"MOVE\"]"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {
    type O = terrars::BlockAssignable<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {}

impl BuildGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {
    pub fn build(self) -> GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {
        GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {
            mode: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigElRef {
        GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nSpecified if fleet logging feature is enabled. Possible values: [\"MODE_UNSPECIFIED\", \"COPY\", \"MOVE\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {
    #[doc =
        "Set the field `mode`.\nSpecified if fleet logging feature is enabled. Possible values: [\"MODE_UNSPECIFIED\", \"COPY\", \"MOVE\"]"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {
    type O = terrars::BlockAssignable<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {}

impl BuildGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {
    pub fn build(self) -> GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {
        GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {
            mode: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigElRef {
        GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nSpecified if fleet logging feature is enabled. Possible values: [\"MODE_UNSPECIFIED\", \"COPY\", \"MOVE\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDynamic {
    default_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl>>,
    fleet_scope_logs_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl>>,
}

#[derive(Serialize)]
pub struct GkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_config: std
    ::option
    ::Option<std::vec::Vec<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleet_scope_logs_config: std
    ::option
    ::Option<std::vec::Vec<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl>>,
    dynamic: GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDynamic,
}

impl GkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {
    #[doc = "Set the field `default_config`.\n"]
    pub fn set_default_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fleet_scope_logs_config`.\n"]
    pub fn set_fleet_scope_logs_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fleet_scope_logs_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fleet_scope_logs_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {
    type O = terrars::BlockAssignable<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {}

impl BuildGkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {
    pub fn build(self) -> GkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {
        GkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {
            default_config: core::default::Default::default(),
            fleet_scope_logs_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElRef {
        GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_config` after provisioning.\n"]
    pub fn default_config(
        &self,
    ) -> terrars::ListRef<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_config", self.base))
    }

    #[doc = "Get a reference to the value of field `fleet_scope_logs_config` after provisioning.\n"]
    pub fn fleet_scope_logs_config(
        &self,
    ) -> terrars::ListRef<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet_scope_logs_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureSpecElFleetobservabilityElDynamic {
    logging_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl>>,
}

#[derive(Serialize)]
pub struct GkeHubFeatureSpecElFleetobservabilityEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl>>,
    dynamic: GkeHubFeatureSpecElFleetobservabilityElDynamic,
}

impl GkeHubFeatureSpecElFleetobservabilityEl {
    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureSpecElFleetobservabilityEl {
    type O = terrars::BlockAssignable<GkeHubFeatureSpecElFleetobservabilityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureSpecElFleetobservabilityEl {}

impl BuildGkeHubFeatureSpecElFleetobservabilityEl {
    pub fn build(self) -> GkeHubFeatureSpecElFleetobservabilityEl {
        GkeHubFeatureSpecElFleetobservabilityEl {
            logging_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureSpecElFleetobservabilityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureSpecElFleetobservabilityElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubFeatureSpecElFleetobservabilityElRef {
        GkeHubFeatureSpecElFleetobservabilityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureSpecElFleetobservabilityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<GkeHubFeatureSpecElFleetobservabilityElLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureSpecElMulticlusteringressEl {
    config_membership: terrars::PrimField<std::string::String>,
}

impl GkeHubFeatureSpecElMulticlusteringressEl { }

impl terrars::ToListMappable for GkeHubFeatureSpecElMulticlusteringressEl {
    type O = terrars::BlockAssignable<GkeHubFeatureSpecElMulticlusteringressEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureSpecElMulticlusteringressEl {
    #[doc =
        "Fully-qualified Membership name which hosts the MultiClusterIngress CRD. Example: 'projects/foo-proj/locations/global/memberships/bar'"]
    pub config_membership: terrars::PrimField<std::string::String>,
}

impl BuildGkeHubFeatureSpecElMulticlusteringressEl {
    pub fn build(self) -> GkeHubFeatureSpecElMulticlusteringressEl {
        GkeHubFeatureSpecElMulticlusteringressEl { config_membership: self.config_membership }
    }
}

pub struct GkeHubFeatureSpecElMulticlusteringressElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureSpecElMulticlusteringressElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubFeatureSpecElMulticlusteringressElRef {
        GkeHubFeatureSpecElMulticlusteringressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureSpecElMulticlusteringressElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `config_membership` after provisioning.\nFully-qualified Membership name which hosts the MultiClusterIngress CRD. Example: 'projects/foo-proj/locations/global/memberships/bar'"]
    pub fn config_membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_membership", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureSpecElDynamic {
    fleetobservability: std::option::Option<terrars::DynamicBlock<GkeHubFeatureSpecElFleetobservabilityEl>>,
    multiclusteringress: std::option::Option<terrars::DynamicBlock<GkeHubFeatureSpecElMulticlusteringressEl>>,
}

#[derive(Serialize)]
pub struct GkeHubFeatureSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleetobservability: std::option::Option<std::vec::Vec<GkeHubFeatureSpecElFleetobservabilityEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multiclusteringress: std::option::Option<std::vec::Vec<GkeHubFeatureSpecElMulticlusteringressEl>>,
    dynamic: GkeHubFeatureSpecElDynamic,
}

impl GkeHubFeatureSpecEl {
    #[doc = "Set the field `fleetobservability`.\n"]
    pub fn set_fleetobservability(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeHubFeatureSpecElFleetobservabilityEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fleetobservability = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fleetobservability = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `multiclusteringress`.\n"]
    pub fn set_multiclusteringress(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeHubFeatureSpecElMulticlusteringressEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.multiclusteringress = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.multiclusteringress = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureSpecEl {
    type O = terrars::BlockAssignable<GkeHubFeatureSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureSpecEl {}

impl BuildGkeHubFeatureSpecEl {
    pub fn build(self) -> GkeHubFeatureSpecEl {
        GkeHubFeatureSpecEl {
            fleetobservability: core::default::Default::default(),
            multiclusteringress: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubFeatureSpecElRef {
        GkeHubFeatureSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fleetobservability` after provisioning.\n"]
    pub fn fleetobservability(&self) -> terrars::ListRef<GkeHubFeatureSpecElFleetobservabilityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleetobservability", self.base))
    }

    #[doc = "Get a reference to the value of field `multiclusteringress` after provisioning.\n"]
    pub fn multiclusteringress(&self) -> terrars::ListRef<GkeHubFeatureSpecElMulticlusteringressElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multiclusteringress", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureTimeoutsEl {
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

impl terrars::ToListMappable for GkeHubFeatureTimeoutsEl {
    type O = terrars::BlockAssignable<GkeHubFeatureTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureTimeoutsEl {}

impl BuildGkeHubFeatureTimeoutsEl {
    pub fn build(self) -> GkeHubFeatureTimeoutsEl {
        GkeHubFeatureTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubFeatureTimeoutsElRef {
        GkeHubFeatureTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureTimeoutsElRef {
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
struct GkeHubFeatureDynamic {
    fleet_default_member_config: std::option::Option<terrars::DynamicBlock<GkeHubFeatureFleetDefaultMemberConfigEl>>,
    spec: std::option::Option<terrars::DynamicBlock<GkeHubFeatureSpecEl>>,
}
