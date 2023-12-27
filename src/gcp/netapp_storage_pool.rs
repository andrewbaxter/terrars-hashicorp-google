use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetappStoragePoolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_directory: Option<PrimField<String>>,
    capacity_gib: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ldap_enabled: Option<PrimField<bool>>,
    location: PrimField<String>,
    name: PrimField<String>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    service_level: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetappStoragePoolTimeoutsEl>,
}

struct NetappStoragePool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetappStoragePoolData>,
}

#[derive(Clone)]
pub struct NetappStoragePool(Rc<NetappStoragePool_>);

impl NetappStoragePool {
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

    #[doc= "Set the field `active_directory`.\nSpecifies the Active Directory policy to be used. Format: 'projects/{{project}}/locations/{{location}}/activeDirectories/{{name}}'.\nThe policy needs to be in the same location as the storage pool."]
    pub fn set_active_directory(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().active_directory = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nAn optional description of this resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `kms_config`.\nSpecifies the CMEK policy to be used for volume encryption. Format: 'projects/{{project}}/locations/{{location}}/kmsConfigs/{{name}}'.\nThe policy needs to be in the same location as the storage pool."]
    pub fn set_kms_config(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_config = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `ldap_enabled`.\nWhen enabled, the volumes uses Active Directory as LDAP name service for UID/GID lookups. Required to enable extended group support for NFSv3,\nusing security identifiers for NFSv4.1 or principal names for kerberized NFSv4.1."]
    pub fn set_ldap_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ldap_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetappStoragePoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `active_directory` after provisioning.\nSpecifies the Active Directory policy to be used. Format: 'projects/{{project}}/locations/{{location}}/activeDirectories/{{name}}'.\nThe policy needs to be in the same location as the storage pool."]
    pub fn active_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.active_directory", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `capacity_gib` after provisioning.\nCapacity of the storage pool (in GiB)."]
    pub fn capacity_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_gib", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encryption_type` after provisioning.\nReports if volumes in the pool are encrypted using a Google-managed encryption key or CMEK."]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_config` after provisioning.\nSpecifies the CMEK policy to be used for volume encryption. Format: 'projects/{{project}}/locations/{{location}}/kmsConfigs/{{name}}'.\nThe policy needs to be in the same location as the storage pool."]
    pub fn kms_config(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ldap_enabled` after provisioning.\nWhen enabled, the volumes uses Active Directory as LDAP name service for UID/GID lookups. Required to enable extended group support for NFSv3,\nusing security identifiers for NFSv4.1 or principal names for kerberized NFSv4.1."]
    pub fn ldap_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ldap_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nName of the location. Usually a region name, expect for some STANDARD service level pools which require a zone name."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the storage pool. Needs to be unique per location."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nVPC network name with format: 'projects/{{project}}/global/networks/{{network}}'"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_level` after provisioning.\nService level of the storage pool. Possible values: [\"PREMIUM\", \"EXTREME\", \"STANDARD\"]"]
    pub fn service_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volume_capacity_gib` after provisioning.\nSize allocated to volumes in the storage pool (in GiB)."]
    pub fn volume_capacity_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_capacity_gib", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volume_count` after provisioning.\nNumber of volume in the storage pool."]
    pub fn volume_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetappStoragePoolTimeoutsElRef {
        NetappStoragePoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for NetappStoragePool {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for NetappStoragePool { }

impl ToListMappable for NetappStoragePool {
    type O = ListRef<NetappStoragePoolRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for NetappStoragePool_ {
    fn extract_resource_type(&self) -> String {
        "google_netapp_storage_pool".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetappStoragePool {
    pub tf_id: String,
    #[doc= "Capacity of the storage pool (in GiB)."]
    pub capacity_gib: PrimField<String>,
    #[doc= "Name of the location. Usually a region name, expect for some STANDARD service level pools which require a zone name."]
    pub location: PrimField<String>,
    #[doc= "The resource name of the storage pool. Needs to be unique per location."]
    pub name: PrimField<String>,
    #[doc= "VPC network name with format: 'projects/{{project}}/global/networks/{{network}}'"]
    pub network: PrimField<String>,
    #[doc= "Service level of the storage pool. Possible values: [\"PREMIUM\", \"EXTREME\", \"STANDARD\"]"]
    pub service_level: PrimField<String>,
}

impl BuildNetappStoragePool {
    pub fn build(self, stack: &mut Stack) -> NetappStoragePool {
        let out = NetappStoragePool(Rc::new(NetappStoragePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetappStoragePoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                active_directory: core::default::Default::default(),
                capacity_gib: self.capacity_gib,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_config: core::default::Default::default(),
                labels: core::default::Default::default(),
                ldap_enabled: core::default::Default::default(),
                location: self.location,
                name: self.name,
                network: self.network,
                project: core::default::Default::default(),
                service_level: self.service_level,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetappStoragePoolRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetappStoragePoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetappStoragePoolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `active_directory` after provisioning.\nSpecifies the Active Directory policy to be used. Format: 'projects/{{project}}/locations/{{location}}/activeDirectories/{{name}}'.\nThe policy needs to be in the same location as the storage pool."]
    pub fn active_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.active_directory", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `capacity_gib` after provisioning.\nCapacity of the storage pool (in GiB)."]
    pub fn capacity_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_gib", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encryption_type` after provisioning.\nReports if volumes in the pool are encrypted using a Google-managed encryption key or CMEK."]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_config` after provisioning.\nSpecifies the CMEK policy to be used for volume encryption. Format: 'projects/{{project}}/locations/{{location}}/kmsConfigs/{{name}}'.\nThe policy needs to be in the same location as the storage pool."]
    pub fn kms_config(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ldap_enabled` after provisioning.\nWhen enabled, the volumes uses Active Directory as LDAP name service for UID/GID lookups. Required to enable extended group support for NFSv3,\nusing security identifiers for NFSv4.1 or principal names for kerberized NFSv4.1."]
    pub fn ldap_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ldap_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nName of the location. Usually a region name, expect for some STANDARD service level pools which require a zone name."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the storage pool. Needs to be unique per location."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nVPC network name with format: 'projects/{{project}}/global/networks/{{network}}'"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_level` after provisioning.\nService level of the storage pool. Possible values: [\"PREMIUM\", \"EXTREME\", \"STANDARD\"]"]
    pub fn service_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volume_capacity_gib` after provisioning.\nSize allocated to volumes in the storage pool (in GiB)."]
    pub fn volume_capacity_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_capacity_gib", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volume_count` after provisioning.\nNumber of volume in the storage pool."]
    pub fn volume_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetappStoragePoolTimeoutsElRef {
        NetappStoragePoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct NetappStoragePoolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl NetappStoragePoolTimeoutsEl {
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

impl ToListMappable for NetappStoragePoolTimeoutsEl {
    type O = BlockAssignable<NetappStoragePoolTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetappStoragePoolTimeoutsEl {}

impl BuildNetappStoragePoolTimeoutsEl {
    pub fn build(self) -> NetappStoragePoolTimeoutsEl {
        NetappStoragePoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct NetappStoragePoolTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetappStoragePoolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetappStoragePoolTimeoutsElRef {
        NetappStoragePoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetappStoragePoolTimeoutsElRef {
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
