use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CloudAssetProjectFeedData {
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
    asset_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asset_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    billing_project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    feed_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<CloudAssetProjectFeedConditionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feed_output_config: std::option::Option<std::vec::Vec<CloudAssetProjectFeedFeedOutputConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudAssetProjectFeedTimeoutsEl>,
    dynamic: CloudAssetProjectFeedDynamic,
}

struct CloudAssetProjectFeed_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudAssetProjectFeedData>,
}

#[derive(Clone)]
pub struct CloudAssetProjectFeed(std::rc::Rc<CloudAssetProjectFeed_>);

impl CloudAssetProjectFeed {
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
        "Set the field `asset_names`.\nA list of the full names of the assets to receive updates. You must specify either or both of\nassetNames and assetTypes. Only asset updates matching specified assetNames and assetTypes are\nexported to the feed. For example: //compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1.\nSee https://cloud.google.com/apis/design/resourceNames#fullResourceName for more info."]
    pub fn set_asset_names(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().asset_names = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `asset_types`.\nA list of types of the assets to receive updates. You must specify either or both of assetNames\nand assetTypes. Only asset updates matching specified assetNames and assetTypes are exported to\nthe feed. For example: \"compute.googleapis.com/Disk\"\nSee https://cloud.google.com/asset-inventory/docs/supported-asset-types for a list of all\nsupported asset types."]
    pub fn set_asset_types(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().asset_types = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `billing_project`.\nThe project whose identity will be used when sending messages to the\ndestination pubsub topic. It also specifies the project for API\nenablement check, quota, and billing. If not specified, the resource's\nproject will be used."]
    pub fn set_billing_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().billing_project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `content_type`.\nAsset content type. If not specified, no content but the asset name and type will be returned. Possible values: [\"CONTENT_TYPE_UNSPECIFIED\", \"RESOURCE\", \"IAM_POLICY\", \"ORG_POLICY\", \"OS_INVENTORY\", \"ACCESS_POLICY\"]"]
    pub fn set_content_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().content_type = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudAssetProjectFeedConditionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `feed_output_config`.\n"]
    pub fn set_feed_output_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudAssetProjectFeedFeedOutputConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().feed_output_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.feed_output_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudAssetProjectFeedTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `asset_names` after provisioning.\nA list of the full names of the assets to receive updates. You must specify either or both of\nassetNames and assetTypes. Only asset updates matching specified assetNames and assetTypes are\nexported to the feed. For example: //compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1.\nSee https://cloud.google.com/apis/design/resourceNames#fullResourceName for more info."]
    pub fn asset_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asset_names", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `asset_types` after provisioning.\nA list of types of the assets to receive updates. You must specify either or both of assetNames\nand assetTypes. Only asset updates matching specified assetNames and assetTypes are exported to\nthe feed. For example: \"compute.googleapis.com/Disk\"\nSee https://cloud.google.com/asset-inventory/docs/supported-asset-types for a list of all\nsupported asset types."]
    pub fn asset_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asset_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `billing_project` after provisioning.\nThe project whose identity will be used when sending messages to the\ndestination pubsub topic. It also specifies the project for API\nenablement check, quota, and billing. If not specified, the resource's\nproject will be used."]
    pub fn billing_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nAsset content type. If not specified, no content but the asset name and type will be returned. Possible values: [\"CONTENT_TYPE_UNSPECIFIED\", \"RESOURCE\", \"IAM_POLICY\", \"ORG_POLICY\", \"OS_INVENTORY\", \"ACCESS_POLICY\"]"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `feed_id` after provisioning.\nThis is the client-assigned asset feed identifier and it needs to be unique under a specific parent."]
    pub fn feed_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feed_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe format will be projects/{projectNumber}/feeds/{client-assigned_feed_identifier}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<CloudAssetProjectFeedConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feed_output_config` after provisioning.\n"]
    pub fn feed_output_config(&self) -> terrars::ListRef<CloudAssetProjectFeedFeedOutputConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.feed_output_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudAssetProjectFeedTimeoutsElRef {
        CloudAssetProjectFeedTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for CloudAssetProjectFeed {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudAssetProjectFeed { }

impl terrars::ToListMappable for CloudAssetProjectFeed {
    type O = terrars::ListRef<CloudAssetProjectFeedRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudAssetProjectFeed_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_cloud_asset_project_feed".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudAssetProjectFeed {
    pub tf_id: std::string::String,
    #[doc = "This is the client-assigned asset feed identifier and it needs to be unique under a specific parent."]
    pub feed_id: terrars::PrimField<std::string::String>,
}

impl BuildCloudAssetProjectFeed {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudAssetProjectFeed {
        let out = CloudAssetProjectFeed(std::rc::Rc::new(CloudAssetProjectFeed_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudAssetProjectFeedData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                asset_names: core::default::Default::default(),
                asset_types: core::default::Default::default(),
                billing_project: core::default::Default::default(),
                content_type: core::default::Default::default(),
                feed_id: self.feed_id,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                condition: core::default::Default::default(),
                feed_output_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudAssetProjectFeedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudAssetProjectFeedRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudAssetProjectFeedRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `asset_names` after provisioning.\nA list of the full names of the assets to receive updates. You must specify either or both of\nassetNames and assetTypes. Only asset updates matching specified assetNames and assetTypes are\nexported to the feed. For example: //compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1.\nSee https://cloud.google.com/apis/design/resourceNames#fullResourceName for more info."]
    pub fn asset_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asset_names", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `asset_types` after provisioning.\nA list of types of the assets to receive updates. You must specify either or both of assetNames\nand assetTypes. Only asset updates matching specified assetNames and assetTypes are exported to\nthe feed. For example: \"compute.googleapis.com/Disk\"\nSee https://cloud.google.com/asset-inventory/docs/supported-asset-types for a list of all\nsupported asset types."]
    pub fn asset_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asset_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `billing_project` after provisioning.\nThe project whose identity will be used when sending messages to the\ndestination pubsub topic. It also specifies the project for API\nenablement check, quota, and billing. If not specified, the resource's\nproject will be used."]
    pub fn billing_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nAsset content type. If not specified, no content but the asset name and type will be returned. Possible values: [\"CONTENT_TYPE_UNSPECIFIED\", \"RESOURCE\", \"IAM_POLICY\", \"ORG_POLICY\", \"OS_INVENTORY\", \"ACCESS_POLICY\"]"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `feed_id` after provisioning.\nThis is the client-assigned asset feed identifier and it needs to be unique under a specific parent."]
    pub fn feed_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feed_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe format will be projects/{projectNumber}/feeds/{client-assigned_feed_identifier}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<CloudAssetProjectFeedConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feed_output_config` after provisioning.\n"]
    pub fn feed_output_config(&self) -> terrars::ListRef<CloudAssetProjectFeedFeedOutputConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.feed_output_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudAssetProjectFeedTimeoutsElRef {
        CloudAssetProjectFeedTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudAssetProjectFeedConditionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudAssetProjectFeedConditionEl {
    #[doc =
        "Set the field `description`.\nDescription of the expression. This is a longer text which describes the expression,\ne.g. when hovered over it in a UI."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a file\nname and a position in the file."]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose.\nThis can be used e.g. in UIs which allow to enter the expression."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudAssetProjectFeedConditionEl {
    type O = terrars::BlockAssignable<CloudAssetProjectFeedConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudAssetProjectFeedConditionEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildCloudAssetProjectFeedConditionEl {
    pub fn build(self) -> CloudAssetProjectFeedConditionEl {
        CloudAssetProjectFeedConditionEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

pub struct CloudAssetProjectFeedConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudAssetProjectFeedConditionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudAssetProjectFeedConditionElRef {
        CloudAssetProjectFeedConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudAssetProjectFeedConditionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the expression. This is a longer text which describes the expression,\ne.g. when hovered over it in a UI."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a file\nname and a position in the file."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose.\nThis can be used e.g. in UIs which allow to enter the expression."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudAssetProjectFeedFeedOutputConfigElPubsubDestinationEl {
    topic: terrars::PrimField<std::string::String>,
}

impl CloudAssetProjectFeedFeedOutputConfigElPubsubDestinationEl { }

impl terrars::ToListMappable for CloudAssetProjectFeedFeedOutputConfigElPubsubDestinationEl {
    type O = terrars::BlockAssignable<CloudAssetProjectFeedFeedOutputConfigElPubsubDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudAssetProjectFeedFeedOutputConfigElPubsubDestinationEl {
    #[doc = "Destination on Cloud Pubsub topic."]
    pub topic: terrars::PrimField<std::string::String>,
}

impl BuildCloudAssetProjectFeedFeedOutputConfigElPubsubDestinationEl {
    pub fn build(self) -> CloudAssetProjectFeedFeedOutputConfigElPubsubDestinationEl {
        CloudAssetProjectFeedFeedOutputConfigElPubsubDestinationEl { topic: self.topic }
    }
}

pub struct CloudAssetProjectFeedFeedOutputConfigElPubsubDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudAssetProjectFeedFeedOutputConfigElPubsubDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudAssetProjectFeedFeedOutputConfigElPubsubDestinationElRef {
        CloudAssetProjectFeedFeedOutputConfigElPubsubDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudAssetProjectFeedFeedOutputConfigElPubsubDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `topic` after provisioning.\nDestination on Cloud Pubsub topic."]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudAssetProjectFeedFeedOutputConfigElDynamic {
    pubsub_destination: std
    ::option
    ::Option<terrars::DynamicBlock<CloudAssetProjectFeedFeedOutputConfigElPubsubDestinationEl>>,
}

#[derive(Serialize)]
pub struct CloudAssetProjectFeedFeedOutputConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pubsub_destination: std::option::Option<std::vec::Vec<CloudAssetProjectFeedFeedOutputConfigElPubsubDestinationEl>>,
    dynamic: CloudAssetProjectFeedFeedOutputConfigElDynamic,
}

impl CloudAssetProjectFeedFeedOutputConfigEl {
    #[doc = "Set the field `pubsub_destination`.\n"]
    pub fn set_pubsub_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudAssetProjectFeedFeedOutputConfigElPubsubDestinationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pubsub_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pubsub_destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudAssetProjectFeedFeedOutputConfigEl {
    type O = terrars::BlockAssignable<CloudAssetProjectFeedFeedOutputConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudAssetProjectFeedFeedOutputConfigEl {}

impl BuildCloudAssetProjectFeedFeedOutputConfigEl {
    pub fn build(self) -> CloudAssetProjectFeedFeedOutputConfigEl {
        CloudAssetProjectFeedFeedOutputConfigEl {
            pubsub_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudAssetProjectFeedFeedOutputConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudAssetProjectFeedFeedOutputConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudAssetProjectFeedFeedOutputConfigElRef {
        CloudAssetProjectFeedFeedOutputConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudAssetProjectFeedFeedOutputConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pubsub_destination` after provisioning.\n"]
    pub fn pubsub_destination(&self) -> terrars::ListRef<CloudAssetProjectFeedFeedOutputConfigElPubsubDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudAssetProjectFeedTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudAssetProjectFeedTimeoutsEl {
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

impl terrars::ToListMappable for CloudAssetProjectFeedTimeoutsEl {
    type O = terrars::BlockAssignable<CloudAssetProjectFeedTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudAssetProjectFeedTimeoutsEl {}

impl BuildCloudAssetProjectFeedTimeoutsEl {
    pub fn build(self) -> CloudAssetProjectFeedTimeoutsEl {
        CloudAssetProjectFeedTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CloudAssetProjectFeedTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudAssetProjectFeedTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudAssetProjectFeedTimeoutsElRef {
        CloudAssetProjectFeedTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudAssetProjectFeedTimeoutsElRef {
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
struct CloudAssetProjectFeedDynamic {
    condition: std::option::Option<terrars::DynamicBlock<CloudAssetProjectFeedConditionEl>>,
    feed_output_config: std::option::Option<terrars::DynamicBlock<CloudAssetProjectFeedFeedOutputConfigEl>>,
}
