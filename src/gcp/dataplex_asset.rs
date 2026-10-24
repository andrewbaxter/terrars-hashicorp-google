use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataplexAssetData {
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
    dataplex_zone: terrars::PrimField<std::string::String>,
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discovery_spec: std::option::Option<std::vec::Vec<DataplexAssetDiscoverySpecEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_spec: std::option::Option<std::vec::Vec<DataplexAssetResourceSpecEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataplexAssetTimeoutsEl>,
    dynamic: DataplexAssetDynamic,
}

struct DataplexAsset_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataplexAssetData>,
}

#[derive(Clone)]
pub struct DataplexAsset(std::rc::Rc<DataplexAsset_>);

impl DataplexAsset {
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

    #[doc = "Set the field `description`.\nOptional. Description of the asset."]
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
        "Set the field `labels`.\nOptional. User defined labels for the asset.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexAssetDiscoverySpecEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexAssetResourceSpecEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataplexAssetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the asset was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dataplex_zone` after provisioning.\nThe zone for the resource"]
    pub fn dataplex_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataplex_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the asset."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discovery_status` after provisioning.\nOutput only. Status of the discovery feature applied to data referenced by this asset."]
    pub fn discovery_status(&self) -> terrars::ListRef<DataplexAssetDiscoveryStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discovery_status", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nOptional. User defined labels for the asset.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the asset."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_status` after provisioning.\nOutput only. Status of the resource referenced by this asset."]
    pub fn resource_status(&self) -> terrars::ListRef<DataplexAssetResourceStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_status` after provisioning.\nOutput only. Status of the security policy applied to resource referenced by this asset."]
    pub fn security_status(&self) -> terrars::ListRef<DataplexAssetSecurityStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. Current state of the asset. Possible values: STATE_UNSPECIFIED, ACTIVE, CREATING, DELETING, ACTION_REQUIRED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. System generated globally unique ID for the asset. This ID will be different if the asset is deleted and re-created with the same name."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the asset was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `discovery_spec` after provisioning.\n"]
    pub fn discovery_spec(&self) -> terrars::ListRef<DataplexAssetDiscoverySpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discovery_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_spec` after provisioning.\n"]
    pub fn resource_spec(&self) -> terrars::ListRef<DataplexAssetResourceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexAssetTimeoutsElRef {
        DataplexAssetTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataplexAsset {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataplexAsset { }

impl terrars::ToListMappable for DataplexAsset {
    type O = terrars::ListRef<DataplexAssetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataplexAsset_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dataplex_asset".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataplexAsset {
    pub tf_id: std::string::String,
    #[doc = "The zone for the resource"]
    pub dataplex_zone: terrars::PrimField<std::string::String>,
    #[doc = "The lake for the resource"]
    pub lake: terrars::PrimField<std::string::String>,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The name of the asset."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataplexAsset {
    pub fn build(self, stack: &mut terrars::Stack) -> DataplexAsset {
        let out = DataplexAsset(std::rc::Rc::new(DataplexAsset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataplexAssetData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                dataplex_zone: self.dataplex_zone,
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                lake: self.lake,
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
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

pub struct DataplexAssetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexAssetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataplexAssetRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the asset was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dataplex_zone` after provisioning.\nThe zone for the resource"]
    pub fn dataplex_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataplex_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the asset."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discovery_status` after provisioning.\nOutput only. Status of the discovery feature applied to data referenced by this asset."]
    pub fn discovery_status(&self) -> terrars::ListRef<DataplexAssetDiscoveryStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discovery_status", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nOptional. User defined labels for the asset.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the asset."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_status` after provisioning.\nOutput only. Status of the resource referenced by this asset."]
    pub fn resource_status(&self) -> terrars::ListRef<DataplexAssetResourceStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_status` after provisioning.\nOutput only. Status of the security policy applied to resource referenced by this asset."]
    pub fn security_status(&self) -> terrars::ListRef<DataplexAssetSecurityStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. Current state of the asset. Possible values: STATE_UNSPECIFIED, ACTIVE, CREATING, DELETING, ACTION_REQUIRED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. System generated globally unique ID for the asset. This ID will be different if the asset is deleted and re-created with the same name."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the asset was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `discovery_spec` after provisioning.\n"]
    pub fn discovery_spec(&self) -> terrars::ListRef<DataplexAssetDiscoverySpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discovery_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_spec` after provisioning.\n"]
    pub fn resource_spec(&self) -> terrars::ListRef<DataplexAssetResourceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexAssetTimeoutsElRef {
        DataplexAssetTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataplexAssetDiscoveryStatusElStatsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filesets: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tables: std::option::Option<terrars::PrimField<f64>>,
}

impl DataplexAssetDiscoveryStatusElStatsEl {
    #[doc = "Set the field `data_items`.\n"]
    pub fn set_data_items(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.data_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_size`.\n"]
    pub fn set_data_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.data_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filesets`.\n"]
    pub fn set_filesets(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.filesets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tables`.\n"]
    pub fn set_tables(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tables = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexAssetDiscoveryStatusElStatsEl {
    type O = terrars::BlockAssignable<DataplexAssetDiscoveryStatusElStatsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexAssetDiscoveryStatusElStatsEl {}

impl BuildDataplexAssetDiscoveryStatusElStatsEl {
    pub fn build(self) -> DataplexAssetDiscoveryStatusElStatsEl {
        DataplexAssetDiscoveryStatusElStatsEl {
            data_items: core::default::Default::default(),
            data_size: core::default::Default::default(),
            filesets: core::default::Default::default(),
            tables: core::default::Default::default(),
        }
    }
}

pub struct DataplexAssetDiscoveryStatusElStatsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexAssetDiscoveryStatusElStatsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexAssetDiscoveryStatusElStatsElRef {
        DataplexAssetDiscoveryStatusElStatsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexAssetDiscoveryStatusElStatsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_items` after provisioning.\n"]
    pub fn data_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_items", self.base))
    }

    #[doc = "Get a reference to the value of field `data_size` after provisioning.\n"]
    pub fn data_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_size", self.base))
    }

    #[doc = "Get a reference to the value of field `filesets` after provisioning.\n"]
    pub fn filesets(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filesets", self.base))
    }

    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tables", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexAssetDiscoveryStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_run_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_run_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stats: std::option::Option<terrars::ListField<DataplexAssetDiscoveryStatusElStatsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexAssetDiscoveryStatusEl {
    #[doc = "Set the field `last_run_duration`.\n"]
    pub fn set_last_run_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_run_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_run_time`.\n"]
    pub fn set_last_run_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_run_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stats`.\n"]
    pub fn set_stats(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataplexAssetDiscoveryStatusElStatsEl>>,
    ) -> Self {
        self.stats = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexAssetDiscoveryStatusEl {
    type O = terrars::BlockAssignable<DataplexAssetDiscoveryStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexAssetDiscoveryStatusEl {}

impl BuildDataplexAssetDiscoveryStatusEl {
    pub fn build(self) -> DataplexAssetDiscoveryStatusEl {
        DataplexAssetDiscoveryStatusEl {
            last_run_duration: core::default::Default::default(),
            last_run_time: core::default::Default::default(),
            message: core::default::Default::default(),
            state: core::default::Default::default(),
            stats: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

pub struct DataplexAssetDiscoveryStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexAssetDiscoveryStatusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexAssetDiscoveryStatusElRef {
        DataplexAssetDiscoveryStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexAssetDiscoveryStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `last_run_duration` after provisioning.\n"]
    pub fn last_run_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_run_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `last_run_time` after provisioning.\n"]
    pub fn last_run_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_run_time", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `stats` after provisioning.\n"]
    pub fn stats(&self) -> terrars::ListRef<DataplexAssetDiscoveryStatusElStatsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stats", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexAssetResourceStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexAssetResourceStatusEl {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexAssetResourceStatusEl {
    type O = terrars::BlockAssignable<DataplexAssetResourceStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexAssetResourceStatusEl {}

impl BuildDataplexAssetResourceStatusEl {
    pub fn build(self) -> DataplexAssetResourceStatusEl {
        DataplexAssetResourceStatusEl {
            message: core::default::Default::default(),
            state: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

pub struct DataplexAssetResourceStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexAssetResourceStatusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexAssetResourceStatusElRef {
        DataplexAssetResourceStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexAssetResourceStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexAssetSecurityStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexAssetSecurityStatusEl {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexAssetSecurityStatusEl {
    type O = terrars::BlockAssignable<DataplexAssetSecurityStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexAssetSecurityStatusEl {}

impl BuildDataplexAssetSecurityStatusEl {
    pub fn build(self) -> DataplexAssetSecurityStatusEl {
        DataplexAssetSecurityStatusEl {
            message: core::default::Default::default(),
            state: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

pub struct DataplexAssetSecurityStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexAssetSecurityStatusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexAssetSecurityStatusElRef {
        DataplexAssetSecurityStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexAssetSecurityStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexAssetDiscoverySpecElCsvOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_type_inference: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_rows: std::option::Option<terrars::PrimField<f64>>,
}

impl DataplexAssetDiscoverySpecElCsvOptionsEl {
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

impl terrars::ToListMappable for DataplexAssetDiscoverySpecElCsvOptionsEl {
    type O = terrars::BlockAssignable<DataplexAssetDiscoverySpecElCsvOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexAssetDiscoverySpecElCsvOptionsEl {}

impl BuildDataplexAssetDiscoverySpecElCsvOptionsEl {
    pub fn build(self) -> DataplexAssetDiscoverySpecElCsvOptionsEl {
        DataplexAssetDiscoverySpecElCsvOptionsEl {
            delimiter: core::default::Default::default(),
            disable_type_inference: core::default::Default::default(),
            encoding: core::default::Default::default(),
            header_rows: core::default::Default::default(),
        }
    }
}

pub struct DataplexAssetDiscoverySpecElCsvOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexAssetDiscoverySpecElCsvOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexAssetDiscoverySpecElCsvOptionsElRef {
        DataplexAssetDiscoverySpecElCsvOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexAssetDiscoverySpecElCsvOptionsElRef {
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
pub struct DataplexAssetDiscoverySpecElJsonOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_type_inference: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexAssetDiscoverySpecElJsonOptionsEl {
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

impl terrars::ToListMappable for DataplexAssetDiscoverySpecElJsonOptionsEl {
    type O = terrars::BlockAssignable<DataplexAssetDiscoverySpecElJsonOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexAssetDiscoverySpecElJsonOptionsEl {}

impl BuildDataplexAssetDiscoverySpecElJsonOptionsEl {
    pub fn build(self) -> DataplexAssetDiscoverySpecElJsonOptionsEl {
        DataplexAssetDiscoverySpecElJsonOptionsEl {
            disable_type_inference: core::default::Default::default(),
            encoding: core::default::Default::default(),
        }
    }
}

pub struct DataplexAssetDiscoverySpecElJsonOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexAssetDiscoverySpecElJsonOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexAssetDiscoverySpecElJsonOptionsElRef {
        DataplexAssetDiscoverySpecElJsonOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexAssetDiscoverySpecElJsonOptionsElRef {
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
struct DataplexAssetDiscoverySpecElDynamic {
    csv_options: std::option::Option<terrars::DynamicBlock<DataplexAssetDiscoverySpecElCsvOptionsEl>>,
    json_options: std::option::Option<terrars::DynamicBlock<DataplexAssetDiscoverySpecElJsonOptionsEl>>,
}

#[derive(Serialize)]
pub struct DataplexAssetDiscoverySpecEl {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_patterns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_patterns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_options: std::option::Option<std::vec::Vec<DataplexAssetDiscoverySpecElCsvOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_options: std::option::Option<std::vec::Vec<DataplexAssetDiscoverySpecElJsonOptionsEl>>,
    dynamic: DataplexAssetDiscoverySpecElDynamic,
}

impl DataplexAssetDiscoverySpecEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexAssetDiscoverySpecElCsvOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexAssetDiscoverySpecElJsonOptionsEl>>,
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

impl terrars::ToListMappable for DataplexAssetDiscoverySpecEl {
    type O = terrars::BlockAssignable<DataplexAssetDiscoverySpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexAssetDiscoverySpecEl {
    #[doc = "Required. Whether discovery is enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildDataplexAssetDiscoverySpecEl {
    pub fn build(self) -> DataplexAssetDiscoverySpecEl {
        DataplexAssetDiscoverySpecEl {
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

pub struct DataplexAssetDiscoverySpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexAssetDiscoverySpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexAssetDiscoverySpecElRef {
        DataplexAssetDiscoverySpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexAssetDiscoverySpecElRef {
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
    pub fn csv_options(&self) -> terrars::ListRef<DataplexAssetDiscoverySpecElCsvOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_options", self.base))
    }

    #[doc = "Get a reference to the value of field `json_options` after provisioning.\n"]
    pub fn json_options(&self) -> terrars::ListRef<DataplexAssetDiscoverySpecElJsonOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_options", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexAssetResourceSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_access_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl DataplexAssetResourceSpecEl {
    #[doc =
        "Set the field `name`.\nImmutable. Relative name of the cloud resource that contains the data that is being managed within a lake. For example: `projects/{project_number}/buckets/{bucket_id}` `projects/{project_number}/datasets/{dataset_id}`"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `read_access_mode`.\nOptional. Determines how read permissions are handled for each asset and their associated tables. Only available to storage buckets assets. Possible values: DIRECT, MANAGED"]
    pub fn set_read_access_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read_access_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexAssetResourceSpecEl {
    type O = terrars::BlockAssignable<DataplexAssetResourceSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexAssetResourceSpecEl {
    #[doc = "Required. Immutable. Type of resource. Possible values: STORAGE_BUCKET, BIGQUERY_DATASET"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDataplexAssetResourceSpecEl {
    pub fn build(self) -> DataplexAssetResourceSpecEl {
        DataplexAssetResourceSpecEl {
            name: core::default::Default::default(),
            read_access_mode: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct DataplexAssetResourceSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexAssetResourceSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexAssetResourceSpecElRef {
        DataplexAssetResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexAssetResourceSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nImmutable. Relative name of the cloud resource that contains the data that is being managed within a lake. For example: `projects/{project_number}/buckets/{bucket_id}` `projects/{project_number}/datasets/{dataset_id}`"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `read_access_mode` after provisioning.\nOptional. Determines how read permissions are handled for each asset and their associated tables. Only available to storage buckets assets. Possible values: DIRECT, MANAGED"]
    pub fn read_access_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_access_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nRequired. Immutable. Type of resource. Possible values: STORAGE_BUCKET, BIGQUERY_DATASET"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexAssetTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexAssetTimeoutsEl {
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

impl terrars::ToListMappable for DataplexAssetTimeoutsEl {
    type O = terrars::BlockAssignable<DataplexAssetTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexAssetTimeoutsEl {}

impl BuildDataplexAssetTimeoutsEl {
    pub fn build(self) -> DataplexAssetTimeoutsEl {
        DataplexAssetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DataplexAssetTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexAssetTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexAssetTimeoutsElRef {
        DataplexAssetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexAssetTimeoutsElRef {
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
struct DataplexAssetDynamic {
    discovery_spec: std::option::Option<terrars::DynamicBlock<DataplexAssetDiscoverySpecEl>>,
    resource_spec: std::option::Option<terrars::DynamicBlock<DataplexAssetResourceSpecEl>>,
}
