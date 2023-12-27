use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataContainerAttachedInstallManifestData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    platform_version: PrimField<String>,
    project: PrimField<String>,
}

struct DataContainerAttachedInstallManifest_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataContainerAttachedInstallManifestData>,
}

#[derive(Clone)]
pub struct DataContainerAttachedInstallManifest(Rc<DataContainerAttachedInstallManifest_>);

impl DataContainerAttachedInstallManifest {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `manifest` after provisioning.\n"]
    pub fn manifest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.manifest", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

impl Referable for DataContainerAttachedInstallManifest {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataContainerAttachedInstallManifest { }

impl ToListMappable for DataContainerAttachedInstallManifest {
    type O = ListRef<DataContainerAttachedInstallManifestRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataContainerAttachedInstallManifest_ {
    fn extract_datasource_type(&self) -> String {
        "google_container_attached_install_manifest".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataContainerAttachedInstallManifest {
    pub tf_id: String,
    #[doc= ""]
    pub cluster_id: PrimField<String>,
    #[doc= ""]
    pub location: PrimField<String>,
    #[doc= ""]
    pub platform_version: PrimField<String>,
    #[doc= ""]
    pub project: PrimField<String>,
}

impl BuildDataContainerAttachedInstallManifest {
    pub fn build(self, stack: &mut Stack) -> DataContainerAttachedInstallManifest {
        let out = DataContainerAttachedInstallManifest(Rc::new(DataContainerAttachedInstallManifest_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataContainerAttachedInstallManifestData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cluster_id: self.cluster_id,
                id: core::default::Default::default(),
                location: self.location,
                platform_version: self.platform_version,
                project: self.project,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataContainerAttachedInstallManifestRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerAttachedInstallManifestRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataContainerAttachedInstallManifestRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `manifest` after provisioning.\n"]
    pub fn manifest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.manifest", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}
