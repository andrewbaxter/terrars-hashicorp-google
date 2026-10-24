use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataplexZoneData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    lake: terrars::PrimField<std::string::String>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discovery_spec: std::option::Option<std::vec::Vec<DataplexZoneDiscoverySpecEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_spec: std::option::Option<std::vec::Vec<DataplexZoneResourceSpecEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataplexZoneTimeoutsEl>,
    dynamic: DataplexZoneDynamic,
}

struct DataplexZone_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataplexZoneData>,
}

#[derive(Clone)]
pub struct DataplexZone(std::rc::Rc<DataplexZone_>);

impl DataplexZone {
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

    #[doc = "Set the field `description`.\nOptional. Description of the zone."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nOptional. User friendly display name."]
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
        "Set the field `labels`.\nOptional. User defined labels for the zone.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discovery_spec`.\n"]
    pub fn set_discovery_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexZoneDiscoverySpecEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().discovery_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.discovery_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_spec`.\n"]
    pub fn set_resource_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexZoneResourceSpecEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataplexZoneTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `asset_status` after provisioning.\nOutput only. Aggregated status of the underlying assets of the zone."]
    pub fn asset_status(&self) -> terrars::ListRef<DataplexZoneAssetStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asset_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the zone was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the zone."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOptional. User friendly display name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nOptional. User defined labels for the zone.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lake` after provisioning.\nThe lake for the resource"]
    pub fn lake(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lake", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the zone."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. Current state of the zone. Possible values: STATE_UNSPECIFIED, ACTIVE, CREATING, DELETING, ACTION_REQUIRED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nRequired. Immutable. The type of the zone. Possible values: TYPE_UNSPECIFIED, RAW, CURATED"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. System generated globally unique ID for the zone. This ID will be different if the zone is deleted and re-created with the same name."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the zone was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `discovery_spec` after provisioning.\n"]
    pub fn discovery_spec(&self) -> terrars::ListRef<DataplexZoneDiscoverySpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discovery_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_spec` after provisioning.\n"]
    pub fn resource_spec(&self) -> terrars::ListRef<DataplexZoneResourceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexZoneTimeoutsElRef {
        DataplexZoneTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataplexZone {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataplexZone { }

impl terrars::ToListMappable for DataplexZone {
    type O = terrars::ListRef<DataplexZoneRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataplexZone_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dataplex_zone".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataplexZone {
    pub tf_id: std::string::String,
    #[doc = "The lake for the resource"]
    pub lake: terrars::PrimField<std::string::String>,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The name of the zone."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Required. Immutable. The type of the zone. Possible values: TYPE_UNSPECIFIED, RAW, CURATED"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDataplexZone {
    pub fn build(self, stack: &mut terrars::Stack) -> DataplexZone {
        let out = DataplexZone(std::rc::Rc::new(DataplexZone_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataplexZoneData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                lake: self.lake,
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                type_: self.type_,
                discovery_spec: core::default::Default::default(),
                resource_spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataplexZoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexZoneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataplexZoneRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `asset_status` after provisioning.\nOutput only. Aggregated status of the underlying assets of the zone."]
    pub fn asset_status(&self) -> terrars::ListRef<DataplexZoneAssetStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asset_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the zone was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the zone."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOptional. User friendly display name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nOptional. User defined labels for the zone.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lake` after provisioning.\nThe lake for the resource"]
    pub fn lake(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lake", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the zone."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. Current state of the zone. Possible values: STATE_UNSPECIFIED, ACTIVE, CREATING, DELETING, ACTION_REQUIRED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nRequired. Immutable. The type of the zone. Possible values: TYPE_UNSPECIFIED, RAW, CURATED"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. System generated globally unique ID for the zone. This ID will be different if the zone is deleted and re-created with the same name."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the zone was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `discovery_spec` after provisioning.\n"]
    pub fn discovery_spec(&self) -> terrars::ListRef<DataplexZoneDiscoverySpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discovery_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_spec` after provisioning.\n"]
    pub fn resource_spec(&self) -> terrars::ListRef<DataplexZoneResourceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexZoneTimeoutsElRef {
        DataplexZoneTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataplexZoneAssetStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active_assets: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_policy_applying_assets: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexZoneAssetStatusEl {
    #[doc = "Set the field `active_assets`.\n"]
    pub fn set_active_assets(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.active_assets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_policy_applying_assets`.\n"]
    pub fn set_security_policy_applying_assets(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.security_policy_applying_assets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexZoneAssetStatusEl {
    type O = terrars::BlockAssignable<DataplexZoneAssetStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexZoneAssetStatusEl {}

impl BuildDataplexZoneAssetStatusEl {
    pub fn build(self) -> DataplexZoneAssetStatusEl {
        DataplexZoneAssetStatusEl {
            active_assets: core::default::Default::default(),
            security_policy_applying_assets: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

pub struct DataplexZoneAssetStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexZoneAssetStatusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexZoneAssetStatusElRef {
        DataplexZoneAssetStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexZoneAssetStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active_assets` after provisioning.\n"]
    pub fn active_assets(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_assets", self.base))
    }

    #[doc = "Get a reference to the value of field `security_policy_applying_assets` after provisioning.\n"]
    pub fn security_policy_applying_assets(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_policy_applying_assets", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexZoneDiscoverySpecElCsvOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_type_inference: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_rows: std::option::Option<terrars::PrimField<f64>>,
}

impl DataplexZoneDiscoverySpecElCsvOptionsEl {
    #[doc = "Set the field `delimiter`.\nOptional. The delimiter being used to separate values. This defaults to ','."]
    pub fn set_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_type_inference`.\nOptional. Whether to disable the inference of data type for CSV data. If true, all columns will be registered as strings."]
    pub fn set_disable_type_inference(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_type_inference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encoding`.\nOptional. The character encoding of the data. The default is UTF-8."]
    pub fn set_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `header_rows`.\nOptional. The number of rows to interpret as header rows that should be skipped when reading data rows."]
    pub fn set_header_rows(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.header_rows = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexZoneDiscoverySpecElCsvOptionsEl {
    type O = terrars::BlockAssignable<DataplexZoneDiscoverySpecElCsvOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexZoneDiscoverySpecElCsvOptionsEl {}

impl BuildDataplexZoneDiscoverySpecElCsvOptionsEl {
    pub fn build(self) -> DataplexZoneDiscoverySpecElCsvOptionsEl {
        DataplexZoneDiscoverySpecElCsvOptionsEl {
            delimiter: core::default::Default::default(),
            disable_type_inference: core::default::Default::default(),
            encoding: core::default::Default::default(),
            header_rows: core::default::Default::default(),
        }
    }
}

pub struct DataplexZoneDiscoverySpecElCsvOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexZoneDiscoverySpecElCsvOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexZoneDiscoverySpecElCsvOptionsElRef {
        DataplexZoneDiscoverySpecElCsvOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexZoneDiscoverySpecElCsvOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `delimiter` after provisioning.\nOptional. The delimiter being used to separate values. This defaults to ','."]
    pub fn delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_type_inference` after provisioning.\nOptional. Whether to disable the inference of data type for CSV data. If true, all columns will be registered as strings."]
    pub fn disable_type_inference(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_type_inference", self.base))
    }

    #[doc =
        "Get a reference to the value of field `encoding` after provisioning.\nOptional. The character encoding of the data. The default is UTF-8."]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `header_rows` after provisioning.\nOptional. The number of rows to interpret as header rows that should be skipped when reading data rows."]
    pub fn header_rows(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_rows", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexZoneDiscoverySpecElJsonOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_type_inference: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexZoneDiscoverySpecElJsonOptionsEl {
    #[doc =
        "Set the field `disable_type_inference`.\nOptional. Whether to disable the inference of data type for Json data. If true, all columns will be registered as their primitive types (strings, number or boolean)."]
    pub fn set_disable_type_inference(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_type_inference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encoding`.\nOptional. The character encoding of the data. The default is UTF-8."]
    pub fn set_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexZoneDiscoverySpecElJsonOptionsEl {
    type O = terrars::BlockAssignable<DataplexZoneDiscoverySpecElJsonOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexZoneDiscoverySpecElJsonOptionsEl {}

impl BuildDataplexZoneDiscoverySpecElJsonOptionsEl {
    pub fn build(self) -> DataplexZoneDiscoverySpecElJsonOptionsEl {
        DataplexZoneDiscoverySpecElJsonOptionsEl {
            disable_type_inference: core::default::Default::default(),
            encoding: core::default::Default::default(),
        }
    }
}

pub struct DataplexZoneDiscoverySpecElJsonOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexZoneDiscoverySpecElJsonOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexZoneDiscoverySpecElJsonOptionsElRef {
        DataplexZoneDiscoverySpecElJsonOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexZoneDiscoverySpecElJsonOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_type_inference` after provisioning.\nOptional. Whether to disable the inference of data type for Json data. If true, all columns will be registered as their primitive types (strings, number or boolean)."]
    pub fn disable_type_inference(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_type_inference", self.base))
    }

    #[doc =
        "Get a reference to the value of field `encoding` after provisioning.\nOptional. The character encoding of the data. The default is UTF-8."]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataplexZoneDiscoverySpecElDynamic {
    csv_options: std::option::Option<terrars::DynamicBlock<DataplexZoneDiscoverySpecElCsvOptionsEl>>,
    json_options: std::option::Option<terrars::DynamicBlock<DataplexZoneDiscoverySpecElJsonOptionsEl>>,
}

#[derive(Serialize)]
pub struct DataplexZoneDiscoverySpecEl {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_patterns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_patterns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_options: std::option::Option<std::vec::Vec<DataplexZoneDiscoverySpecElCsvOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_options: std::option::Option<std::vec::Vec<DataplexZoneDiscoverySpecElJsonOptionsEl>>,
    dynamic: DataplexZoneDiscoverySpecElDynamic,
}

impl DataplexZoneDiscoverySpecEl {
    #[doc =
        "Set the field `exclude_patterns`.\nOptional. The list of patterns to apply for selecting data to exclude during discovery. For Cloud Storage bucket assets, these are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these are interpreted as patterns to match table names."]
    pub fn set_exclude_patterns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_patterns = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_patterns`.\nOptional. The list of patterns to apply for selecting data to include during discovery if only a subset of the data should considered. For Cloud Storage bucket assets, these are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these are interpreted as patterns to match table names."]
    pub fn set_include_patterns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_patterns = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `schedule`.\nOptional. Cron schedule (https://en.wikipedia.org/wiki/Cron) for running discovery periodically. Successive discovery runs must be scheduled at least 60 minutes apart. The default value is to run discovery every 60 minutes. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: \"CRON_TZ=${IANA_TIME_ZONE}\" or TZ=${IANA_TIME_ZONE}\". The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, \"CRON_TZ=America/New_York 1 * * * *\", or \"TZ=America/New_York 1 * * * *\"."]
    pub fn set_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `csv_options`.\n"]
    pub fn set_csv_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexZoneDiscoverySpecElCsvOptionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.csv_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.csv_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `json_options`.\n"]
    pub fn set_json_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexZoneDiscoverySpecElJsonOptionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.json_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.json_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexZoneDiscoverySpecEl {
    type O = terrars::BlockAssignable<DataplexZoneDiscoverySpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexZoneDiscoverySpecEl {
    #[doc = "Required. Whether discovery is enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildDataplexZoneDiscoverySpecEl {
    pub fn build(self) -> DataplexZoneDiscoverySpecEl {
        DataplexZoneDiscoverySpecEl {
            enabled: self.enabled,
            exclude_patterns: core::default::Default::default(),
            include_patterns: core::default::Default::default(),
            schedule: core::default::Default::default(),
            csv_options: core::default::Default::default(),
            json_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataplexZoneDiscoverySpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexZoneDiscoverySpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexZoneDiscoverySpecElRef {
        DataplexZoneDiscoverySpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexZoneDiscoverySpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nRequired. Whether discovery is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclude_patterns` after provisioning.\nOptional. The list of patterns to apply for selecting data to exclude during discovery. For Cloud Storage bucket assets, these are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these are interpreted as patterns to match table names."]
    pub fn exclude_patterns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_patterns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_patterns` after provisioning.\nOptional. The list of patterns to apply for selecting data to include during discovery if only a subset of the data should considered. For Cloud Storage bucket assets, these are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these are interpreted as patterns to match table names."]
    pub fn include_patterns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_patterns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nOptional. Cron schedule (https://en.wikipedia.org/wiki/Cron) for running discovery periodically. Successive discovery runs must be scheduled at least 60 minutes apart. The default value is to run discovery every 60 minutes. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: \"CRON_TZ=${IANA_TIME_ZONE}\" or TZ=${IANA_TIME_ZONE}\". The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, \"CRON_TZ=America/New_York 1 * * * *\", or \"TZ=America/New_York 1 * * * *\"."]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `csv_options` after provisioning.\n"]
    pub fn csv_options(&self) -> terrars::ListRef<DataplexZoneDiscoverySpecElCsvOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_options", self.base))
    }

    #[doc = "Get a reference to the value of field `json_options` after provisioning.\n"]
    pub fn json_options(&self) -> terrars::ListRef<DataplexZoneDiscoverySpecElJsonOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_options", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexZoneResourceSpecEl {
    location_type: terrars::PrimField<std::string::String>,
}

impl DataplexZoneResourceSpecEl { }

impl terrars::ToListMappable for DataplexZoneResourceSpecEl {
    type O = terrars::BlockAssignable<DataplexZoneResourceSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexZoneResourceSpecEl {
    #[doc =
        "Required. Immutable. The location type of the resources that are allowed to be attached to the assets within this zone. Possible values: LOCATION_TYPE_UNSPECIFIED, SINGLE_REGION, MULTI_REGION"]
    pub location_type: terrars::PrimField<std::string::String>,
}

impl BuildDataplexZoneResourceSpecEl {
    pub fn build(self) -> DataplexZoneResourceSpecEl {
        DataplexZoneResourceSpecEl { location_type: self.location_type }
    }
}

pub struct DataplexZoneResourceSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexZoneResourceSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexZoneResourceSpecElRef {
        DataplexZoneResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexZoneResourceSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `location_type` after provisioning.\nRequired. Immutable. The location type of the resources that are allowed to be attached to the assets within this zone. Possible values: LOCATION_TYPE_UNSPECIFIED, SINGLE_REGION, MULTI_REGION"]
    pub fn location_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexZoneTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexZoneTimeoutsEl {
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

impl terrars::ToListMappable for DataplexZoneTimeoutsEl {
    type O = terrars::BlockAssignable<DataplexZoneTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexZoneTimeoutsEl {}

impl BuildDataplexZoneTimeoutsEl {
    pub fn build(self) -> DataplexZoneTimeoutsEl {
        DataplexZoneTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DataplexZoneTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexZoneTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexZoneTimeoutsElRef {
        DataplexZoneTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexZoneTimeoutsElRef {
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
struct DataplexZoneDynamic {
    discovery_spec: std::option::Option<terrars::DynamicBlock<DataplexZoneDiscoverySpecEl>>,
    resource_spec: std::option::Option<terrars::DynamicBlock<DataplexZoneResourceSpecEl>>,
}
