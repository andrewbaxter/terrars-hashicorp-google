use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CloudAssetFolderFeedData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_types: Option<ListField<PrimField<String>>>,
    billing_project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    feed_id: PrimField<String>,
    folder: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<CloudAssetFolderFeedConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feed_output_config: Option<Vec<CloudAssetFolderFeedFeedOutputConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudAssetFolderFeedTimeoutsEl>,
    dynamic: CloudAssetFolderFeedDynamic,
}

struct CloudAssetFolderFeed_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudAssetFolderFeedData>,
}

#[derive(Clone)]
pub struct CloudAssetFolderFeed(Rc<CloudAssetFolderFeed_>);

impl CloudAssetFolderFeed {
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

    #[doc= "Set the field `asset_names`.\nA list of the full names of the assets to receive updates. You must specify either or both of\nassetNames and assetTypes. Only asset updates matching specified assetNames and assetTypes are\nexported to the feed. For example: //compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1.\nSee https://cloud.google.com/apis/design/resourceNames#fullResourceName for more info."]
    pub fn set_asset_names(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().asset_names = Some(v.into());
        self
    }

    #[doc= "Set the field `asset_types`.\nA list of types of the assets to receive updates. You must specify either or both of assetNames\nand assetTypes. Only asset updates matching specified assetNames and assetTypes are exported to\nthe feed. For example: \"compute.googleapis.com/Disk\"\nSee https://cloud.google.com/asset-inventory/docs/supported-asset-types for a list of all\nsupported asset types."]
    pub fn set_asset_types(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().asset_types = Some(v.into());
        self
    }

    #[doc= "Set the field `content_type`.\nAsset content type. If not specified, no content but the asset name and type will be returned. Possible values: [\"CONTENT_TYPE_UNSPECIFIED\", \"RESOURCE\", \"IAM_POLICY\", \"ORG_POLICY\", \"OS_INVENTORY\", \"ACCESS_POLICY\"]"]
    pub fn set_content_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_type = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `condition`.\n"]
    pub fn set_condition(self, v: impl Into<BlockAssignable<CloudAssetFolderFeedConditionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `feed_output_config`.\n"]
    pub fn set_feed_output_config(self, v: impl Into<BlockAssignable<CloudAssetFolderFeedFeedOutputConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().feed_output_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.feed_output_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudAssetFolderFeedTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `asset_names` after provisioning.\nA list of the full names of the assets to receive updates. You must specify either or both of\nassetNames and assetTypes. Only asset updates matching specified assetNames and assetTypes are\nexported to the feed. For example: //compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1.\nSee https://cloud.google.com/apis/design/resourceNames#fullResourceName for more info."]
    pub fn asset_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.asset_names", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `asset_types` after provisioning.\nA list of types of the assets to receive updates. You must specify either or both of assetNames\nand assetTypes. Only asset updates matching specified assetNames and assetTypes are exported to\nthe feed. For example: \"compute.googleapis.com/Disk\"\nSee https://cloud.google.com/asset-inventory/docs/supported-asset-types for a list of all\nsupported asset types."]
    pub fn asset_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.asset_types", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `billing_project` after provisioning.\nThe project whose identity will be used when sending messages to the\ndestination pubsub topic. It also specifies the project for API\nenablement check, quota, and billing."]
    pub fn billing_project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.billing_project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_type` after provisioning.\nAsset content type. If not specified, no content but the asset name and type will be returned. Possible values: [\"CONTENT_TYPE_UNSPECIFIED\", \"RESOURCE\", \"IAM_POLICY\", \"ORG_POLICY\", \"OS_INVENTORY\", \"ACCESS_POLICY\"]"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `feed_id` after provisioning.\nThis is the client-assigned asset feed identifier and it needs to be unique under a specific parent."]
    pub fn feed_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.feed_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `folder` after provisioning.\nThe folder this feed should be created in."]
    pub fn folder(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.folder", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `folder_id` after provisioning.\nThe ID of the folder where this feed has been created. Both [FOLDER_NUMBER]\nand folders/[FOLDER_NUMBER] are accepted."]
    pub fn folder_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.folder_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe format will be folders/{folder_number}/feeds/{client-assigned_feed_identifier}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<CloudAssetFolderFeedConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `feed_output_config` after provisioning.\n"]
    pub fn feed_output_config(&self) -> ListRef<CloudAssetFolderFeedFeedOutputConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.feed_output_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudAssetFolderFeedTimeoutsElRef {
        CloudAssetFolderFeedTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for CloudAssetFolderFeed {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CloudAssetFolderFeed { }

impl ToListMappable for CloudAssetFolderFeed {
    type O = ListRef<CloudAssetFolderFeedRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudAssetFolderFeed_ {
    fn extract_resource_type(&self) -> String {
        "google_cloud_asset_folder_feed".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudAssetFolderFeed {
    pub tf_id: String,
    #[doc= "The project whose identity will be used when sending messages to the\ndestination pubsub topic. It also specifies the project for API\nenablement check, quota, and billing."]
    pub billing_project: PrimField<String>,
    #[doc= "This is the client-assigned asset feed identifier and it needs to be unique under a specific parent."]
    pub feed_id: PrimField<String>,
    #[doc= "The folder this feed should be created in."]
    pub folder: PrimField<String>,
}

impl BuildCloudAssetFolderFeed {
    pub fn build(self, stack: &mut Stack) -> CloudAssetFolderFeed {
        let out = CloudAssetFolderFeed(Rc::new(CloudAssetFolderFeed_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudAssetFolderFeedData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                asset_names: core::default::Default::default(),
                asset_types: core::default::Default::default(),
                billing_project: self.billing_project,
                content_type: core::default::Default::default(),
                feed_id: self.feed_id,
                folder: self.folder,
                id: core::default::Default::default(),
                condition: core::default::Default::default(),
                feed_output_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudAssetFolderFeedRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudAssetFolderFeedRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudAssetFolderFeedRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `asset_names` after provisioning.\nA list of the full names of the assets to receive updates. You must specify either or both of\nassetNames and assetTypes. Only asset updates matching specified assetNames and assetTypes are\nexported to the feed. For example: //compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1.\nSee https://cloud.google.com/apis/design/resourceNames#fullResourceName for more info."]
    pub fn asset_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.asset_names", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `asset_types` after provisioning.\nA list of types of the assets to receive updates. You must specify either or both of assetNames\nand assetTypes. Only asset updates matching specified assetNames and assetTypes are exported to\nthe feed. For example: \"compute.googleapis.com/Disk\"\nSee https://cloud.google.com/asset-inventory/docs/supported-asset-types for a list of all\nsupported asset types."]
    pub fn asset_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.asset_types", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `billing_project` after provisioning.\nThe project whose identity will be used when sending messages to the\ndestination pubsub topic. It also specifies the project for API\nenablement check, quota, and billing."]
    pub fn billing_project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.billing_project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_type` after provisioning.\nAsset content type. If not specified, no content but the asset name and type will be returned. Possible values: [\"CONTENT_TYPE_UNSPECIFIED\", \"RESOURCE\", \"IAM_POLICY\", \"ORG_POLICY\", \"OS_INVENTORY\", \"ACCESS_POLICY\"]"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `feed_id` after provisioning.\nThis is the client-assigned asset feed identifier and it needs to be unique under a specific parent."]
    pub fn feed_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.feed_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `folder` after provisioning.\nThe folder this feed should be created in."]
    pub fn folder(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.folder", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `folder_id` after provisioning.\nThe ID of the folder where this feed has been created. Both [FOLDER_NUMBER]\nand folders/[FOLDER_NUMBER] are accepted."]
    pub fn folder_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.folder_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe format will be folders/{folder_number}/feeds/{client-assigned_feed_identifier}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<CloudAssetFolderFeedConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `feed_output_config` after provisioning.\n"]
    pub fn feed_output_config(&self) -> ListRef<CloudAssetFolderFeedFeedOutputConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.feed_output_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudAssetFolderFeedTimeoutsElRef {
        CloudAssetFolderFeedTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudAssetFolderFeedConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}

impl CloudAssetFolderFeedConditionEl {
    #[doc= "Set the field `description`.\nDescription of the expression. This is a longer text which describes the expression,\ne.g. when hovered over it in a UI."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a file\nname and a position in the file."]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }

    #[doc= "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose.\nThis can be used e.g. in UIs which allow to enter the expression."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}

impl ToListMappable for CloudAssetFolderFeedConditionEl {
    type O = BlockAssignable<CloudAssetFolderFeedConditionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudAssetFolderFeedConditionEl {
    #[doc= "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: PrimField<String>,
}

impl BuildCloudAssetFolderFeedConditionEl {
    pub fn build(self) -> CloudAssetFolderFeedConditionEl {
        CloudAssetFolderFeedConditionEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

pub struct CloudAssetFolderFeedConditionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudAssetFolderFeedConditionElRef {
    fn new(shared: StackShared, base: String) -> CloudAssetFolderFeedConditionElRef {
        CloudAssetFolderFeedConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudAssetFolderFeedConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the expression. This is a longer text which describes the expression,\ne.g. when hovered over it in a UI."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a file\nname and a position in the file."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose.\nThis can be used e.g. in UIs which allow to enter the expression."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudAssetFolderFeedFeedOutputConfigElPubsubDestinationEl {
    topic: PrimField<String>,
}

impl CloudAssetFolderFeedFeedOutputConfigElPubsubDestinationEl { }

impl ToListMappable for CloudAssetFolderFeedFeedOutputConfigElPubsubDestinationEl {
    type O = BlockAssignable<CloudAssetFolderFeedFeedOutputConfigElPubsubDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudAssetFolderFeedFeedOutputConfigElPubsubDestinationEl {
    #[doc= "Destination on Cloud Pubsub topic."]
    pub topic: PrimField<String>,
}

impl BuildCloudAssetFolderFeedFeedOutputConfigElPubsubDestinationEl {
    pub fn build(self) -> CloudAssetFolderFeedFeedOutputConfigElPubsubDestinationEl {
        CloudAssetFolderFeedFeedOutputConfigElPubsubDestinationEl { topic: self.topic }
    }
}

pub struct CloudAssetFolderFeedFeedOutputConfigElPubsubDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudAssetFolderFeedFeedOutputConfigElPubsubDestinationElRef {
    fn new(shared: StackShared, base: String) -> CloudAssetFolderFeedFeedOutputConfigElPubsubDestinationElRef {
        CloudAssetFolderFeedFeedOutputConfigElPubsubDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudAssetFolderFeedFeedOutputConfigElPubsubDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `topic` after provisioning.\nDestination on Cloud Pubsub topic."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudAssetFolderFeedFeedOutputConfigElDynamic {
    pubsub_destination: Option<DynamicBlock<CloudAssetFolderFeedFeedOutputConfigElPubsubDestinationEl>>,
}

#[derive(Serialize)]
pub struct CloudAssetFolderFeedFeedOutputConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pubsub_destination: Option<Vec<CloudAssetFolderFeedFeedOutputConfigElPubsubDestinationEl>>,
    dynamic: CloudAssetFolderFeedFeedOutputConfigElDynamic,
}

impl CloudAssetFolderFeedFeedOutputConfigEl {
    #[doc= "Set the field `pubsub_destination`.\n"]
    pub fn set_pubsub_destination(
        mut self,
        v: impl Into<BlockAssignable<CloudAssetFolderFeedFeedOutputConfigElPubsubDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pubsub_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pubsub_destination = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CloudAssetFolderFeedFeedOutputConfigEl {
    type O = BlockAssignable<CloudAssetFolderFeedFeedOutputConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudAssetFolderFeedFeedOutputConfigEl {}

impl BuildCloudAssetFolderFeedFeedOutputConfigEl {
    pub fn build(self) -> CloudAssetFolderFeedFeedOutputConfigEl {
        CloudAssetFolderFeedFeedOutputConfigEl {
            pubsub_destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudAssetFolderFeedFeedOutputConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudAssetFolderFeedFeedOutputConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudAssetFolderFeedFeedOutputConfigElRef {
        CloudAssetFolderFeedFeedOutputConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudAssetFolderFeedFeedOutputConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `pubsub_destination` after provisioning.\n"]
    pub fn pubsub_destination(&self) -> ListRef<CloudAssetFolderFeedFeedOutputConfigElPubsubDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pubsub_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudAssetFolderFeedTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl CloudAssetFolderFeedTimeoutsEl {
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

impl ToListMappable for CloudAssetFolderFeedTimeoutsEl {
    type O = BlockAssignable<CloudAssetFolderFeedTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudAssetFolderFeedTimeoutsEl {}

impl BuildCloudAssetFolderFeedTimeoutsEl {
    pub fn build(self) -> CloudAssetFolderFeedTimeoutsEl {
        CloudAssetFolderFeedTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CloudAssetFolderFeedTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudAssetFolderFeedTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudAssetFolderFeedTimeoutsElRef {
        CloudAssetFolderFeedTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudAssetFolderFeedTimeoutsElRef {
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
struct CloudAssetFolderFeedDynamic {
    condition: Option<DynamicBlock<CloudAssetFolderFeedConditionEl>>,
    feed_output_config: Option<DynamicBlock<CloudAssetFolderFeedFeedOutputConfigEl>>,
}
