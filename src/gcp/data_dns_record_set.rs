use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataDnsRecordSetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    managed_zone: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

struct DataDnsRecordSet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDnsRecordSetData>,
}

#[derive(Clone)]
pub struct DataDnsRecordSet(Rc<DataDnsRecordSet_>);

impl DataDnsRecordSet {
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

    #[doc= "Get a reference to the value of field `id` after provisioning.\nDNS record set identifier"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `managed_zone` after provisioning.\nThe Name of the zone."]
    pub fn managed_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.managed_zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe DNS name for the resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project for the Google Cloud."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rrdatas` after provisioning.\nThe string data for the records in this record set."]
    pub fn rrdatas(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.rrdatas", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\nThe time-to-live of this record set (seconds)."]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe identifier of a supported record type. See the list of Supported DNS record types."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl Referable for DataDnsRecordSet {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataDnsRecordSet { }

impl ToListMappable for DataDnsRecordSet {
    type O = ListRef<DataDnsRecordSetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataDnsRecordSet_ {
    fn extract_datasource_type(&self) -> String {
        "google_dns_record_set".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDnsRecordSet {
    pub tf_id: String,
    #[doc= "The Name of the zone."]
    pub managed_zone: PrimField<String>,
    #[doc= "The DNS name for the resource."]
    pub name: PrimField<String>,
    #[doc= "The identifier of a supported record type. See the list of Supported DNS record types."]
    pub type_: PrimField<String>,
}

impl BuildDataDnsRecordSet {
    pub fn build(self, stack: &mut Stack) -> DataDnsRecordSet {
        let out = DataDnsRecordSet(Rc::new(DataDnsRecordSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDnsRecordSetData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                managed_zone: self.managed_zone,
                name: self.name,
                project: core::default::Default::default(),
                type_: self.type_,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDnsRecordSetRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDnsRecordSetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDnsRecordSetRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nDNS record set identifier"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `managed_zone` after provisioning.\nThe Name of the zone."]
    pub fn managed_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.managed_zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe DNS name for the resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project for the Google Cloud."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rrdatas` after provisioning.\nThe string data for the records in this record set."]
    pub fn rrdatas(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.rrdatas", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\nThe time-to-live of this record set (seconds)."]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe identifier of a supported record type. See the list of Supported DNS record types."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}
