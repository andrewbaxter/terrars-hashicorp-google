use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataDnsManagedZoneData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}

struct DataDnsManagedZone_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDnsManagedZoneData>,
}

#[derive(Clone)]
pub struct DataDnsManagedZone(Rc<DataDnsManagedZone_>);

impl DataDnsManagedZone {
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

    #[doc= "Set the field `project`.\nThe ID of the project for the Google Cloud."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA textual description field."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns_name` after provisioning.\nThe fully qualified DNS name of this zone."]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nDNS managed zone identifier"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `managed_zone_id` after provisioning.\nUnique identifier for the resource; defined by the server."]
    pub fn managed_zone_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.managed_zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA unique name for the resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name_servers` after provisioning.\nThe list of nameservers that will be authoritative for this domain. Use NS records to redirect from your DNS provider to these names, thus making Google Cloud DNS authoritative for this zone."]
    pub fn name_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.name_servers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project for the Google Cloud."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility` after provisioning.\nThe zone's visibility: public zones are exposed to the Internet, while private zones are visible only to Virtual Private Cloud resources."]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }
}

impl Referable for DataDnsManagedZone {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataDnsManagedZone { }

impl ToListMappable for DataDnsManagedZone {
    type O = ListRef<DataDnsManagedZoneRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataDnsManagedZone_ {
    fn extract_datasource_type(&self) -> String {
        "google_dns_managed_zone".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDnsManagedZone {
    pub tf_id: String,
    #[doc= "A unique name for the resource."]
    pub name: PrimField<String>,
}

impl BuildDataDnsManagedZone {
    pub fn build(self, stack: &mut Stack) -> DataDnsManagedZone {
        let out = DataDnsManagedZone(Rc::new(DataDnsManagedZone_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDnsManagedZoneData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDnsManagedZoneRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDnsManagedZoneRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDnsManagedZoneRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA textual description field."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns_name` after provisioning.\nThe fully qualified DNS name of this zone."]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nDNS managed zone identifier"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `managed_zone_id` after provisioning.\nUnique identifier for the resource; defined by the server."]
    pub fn managed_zone_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.managed_zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA unique name for the resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name_servers` after provisioning.\nThe list of nameservers that will be authoritative for this domain. Use NS records to redirect from your DNS provider to these names, thus making Google Cloud DNS authoritative for this zone."]
    pub fn name_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.name_servers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project for the Google Cloud."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility` after provisioning.\nThe zone's visibility: public zones are exposed to the Internet, while private zones are visible only to Virtual Private Cloud resources."]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }
}
