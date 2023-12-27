use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigqueryReservationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    concurrency: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_idle_slots: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_region_auxiliary: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    slot_capacity: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscale: Option<Vec<BigqueryReservationAutoscaleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigqueryReservationTimeoutsEl>,
    dynamic: BigqueryReservationDynamic,
}

struct BigqueryReservation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigqueryReservationData>,
}

#[derive(Clone)]
pub struct BigqueryReservation(Rc<BigqueryReservation_>);

impl BigqueryReservation {
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

    #[doc= "Set the field `concurrency`.\nMaximum number of queries that are allowed to run concurrently in this reservation. This is a soft limit due to asynchronous nature of the system and various optimizations for small queries. Default value is 0 which means that concurrency will be automatically set based on the reservation size."]
    pub fn set_concurrency(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().concurrency = Some(v.into());
        self
    }

    #[doc= "Set the field `edition`.\nThe edition type. Valid values are STANDARD, ENTERPRISE, ENTERPRISE_PLUS"]
    pub fn set_edition(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().edition = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `ignore_idle_slots`.\nIf false, any query using this reservation will use idle slots from other reservations within\nthe same admin project. If true, a query using this reservation will execute with the slot\ncapacity specified above at most."]
    pub fn set_ignore_idle_slots(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ignore_idle_slots = Some(v.into());
        self
    }

    #[doc= "Set the field `location`.\nThe geographic location where the transfer config should reside.\nExamples: US, EU, asia-northeast1. The default value is US."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }

    #[doc= "Set the field `multi_region_auxiliary`.\nApplicable only for reservations located within one of the BigQuery multi-regions (US or EU).\nIf set to true, this reservation is placed in the organization's secondary region which is designated for disaster recovery purposes. If false, this reservation is placed in the organization's default region."]
    pub fn set_multi_region_auxiliary(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multi_region_auxiliary = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `autoscale`.\n"]
    pub fn set_autoscale(self, v: impl Into<BlockAssignable<BigqueryReservationAutoscaleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autoscale = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autoscale = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigqueryReservationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `concurrency` after provisioning.\nMaximum number of queries that are allowed to run concurrently in this reservation. This is a soft limit due to asynchronous nature of the system and various optimizations for small queries. Default value is 0 which means that concurrency will be automatically set based on the reservation size."]
    pub fn concurrency(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.concurrency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `edition` after provisioning.\nThe edition type. Valid values are STANDARD, ENTERPRISE, ENTERPRISE_PLUS"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.edition", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ignore_idle_slots` after provisioning.\nIf false, any query using this reservation will use idle slots from other reservations within\nthe same admin project. If true, a query using this reservation will execute with the slot\ncapacity specified above at most."]
    pub fn ignore_idle_slots(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_idle_slots", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the transfer config should reside.\nExamples: US, EU, asia-northeast1. The default value is US."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `multi_region_auxiliary` after provisioning.\nApplicable only for reservations located within one of the BigQuery multi-regions (US or EU).\nIf set to true, this reservation is placed in the organization's secondary region which is designated for disaster recovery purposes. If false, this reservation is placed in the organization's default region."]
    pub fn multi_region_auxiliary(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_region_auxiliary", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the reservation. This field must only contain alphanumeric characters or dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slot_capacity` after provisioning.\nMinimum slots available to this reservation. A slot is a unit of computational power in BigQuery, and serves as the\nunit of parallelism. Queries using this reservation might use more slots during runtime if ignoreIdleSlots is set to false."]
    pub fn slot_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.slot_capacity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `autoscale` after provisioning.\n"]
    pub fn autoscale(&self) -> ListRef<BigqueryReservationAutoscaleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.autoscale", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryReservationTimeoutsElRef {
        BigqueryReservationTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for BigqueryReservation {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BigqueryReservation { }

impl ToListMappable for BigqueryReservation {
    type O = ListRef<BigqueryReservationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BigqueryReservation_ {
    fn extract_resource_type(&self) -> String {
        "google_bigquery_reservation".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigqueryReservation {
    pub tf_id: String,
    #[doc= "The name of the reservation. This field must only contain alphanumeric characters or dash."]
    pub name: PrimField<String>,
    #[doc= "Minimum slots available to this reservation. A slot is a unit of computational power in BigQuery, and serves as the\nunit of parallelism. Queries using this reservation might use more slots during runtime if ignoreIdleSlots is set to false."]
    pub slot_capacity: PrimField<f64>,
}

impl BuildBigqueryReservation {
    pub fn build(self, stack: &mut Stack) -> BigqueryReservation {
        let out = BigqueryReservation(Rc::new(BigqueryReservation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigqueryReservationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                concurrency: core::default::Default::default(),
                edition: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_idle_slots: core::default::Default::default(),
                location: core::default::Default::default(),
                multi_region_auxiliary: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                slot_capacity: self.slot_capacity,
                autoscale: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigqueryReservationRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryReservationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigqueryReservationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `concurrency` after provisioning.\nMaximum number of queries that are allowed to run concurrently in this reservation. This is a soft limit due to asynchronous nature of the system and various optimizations for small queries. Default value is 0 which means that concurrency will be automatically set based on the reservation size."]
    pub fn concurrency(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.concurrency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `edition` after provisioning.\nThe edition type. Valid values are STANDARD, ENTERPRISE, ENTERPRISE_PLUS"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.edition", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ignore_idle_slots` after provisioning.\nIf false, any query using this reservation will use idle slots from other reservations within\nthe same admin project. If true, a query using this reservation will execute with the slot\ncapacity specified above at most."]
    pub fn ignore_idle_slots(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_idle_slots", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the transfer config should reside.\nExamples: US, EU, asia-northeast1. The default value is US."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `multi_region_auxiliary` after provisioning.\nApplicable only for reservations located within one of the BigQuery multi-regions (US or EU).\nIf set to true, this reservation is placed in the organization's secondary region which is designated for disaster recovery purposes. If false, this reservation is placed in the organization's default region."]
    pub fn multi_region_auxiliary(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_region_auxiliary", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the reservation. This field must only contain alphanumeric characters or dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slot_capacity` after provisioning.\nMinimum slots available to this reservation. A slot is a unit of computational power in BigQuery, and serves as the\nunit of parallelism. Queries using this reservation might use more slots during runtime if ignoreIdleSlots is set to false."]
    pub fn slot_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.slot_capacity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `autoscale` after provisioning.\n"]
    pub fn autoscale(&self) -> ListRef<BigqueryReservationAutoscaleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.autoscale", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryReservationTimeoutsElRef {
        BigqueryReservationTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BigqueryReservationAutoscaleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_slots: Option<PrimField<f64>>,
}

impl BigqueryReservationAutoscaleEl {
    #[doc= "Set the field `max_slots`.\nNumber of slots to be scaled when needed."]
    pub fn set_max_slots(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_slots = Some(v.into());
        self
    }
}

impl ToListMappable for BigqueryReservationAutoscaleEl {
    type O = BlockAssignable<BigqueryReservationAutoscaleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryReservationAutoscaleEl {}

impl BuildBigqueryReservationAutoscaleEl {
    pub fn build(self) -> BigqueryReservationAutoscaleEl {
        BigqueryReservationAutoscaleEl { max_slots: core::default::Default::default() }
    }
}

pub struct BigqueryReservationAutoscaleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryReservationAutoscaleElRef {
    fn new(shared: StackShared, base: String) -> BigqueryReservationAutoscaleElRef {
        BigqueryReservationAutoscaleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryReservationAutoscaleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `current_slots` after provisioning.\nThe slot capacity added to this reservation when autoscale happens. Will be between [0, max_slots]."]
    pub fn current_slots(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.current_slots", self.base))
    }

    #[doc= "Get a reference to the value of field `max_slots` after provisioning.\nNumber of slots to be scaled when needed."]
    pub fn max_slots(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_slots", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryReservationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl BigqueryReservationTimeoutsEl {
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

impl ToListMappable for BigqueryReservationTimeoutsEl {
    type O = BlockAssignable<BigqueryReservationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryReservationTimeoutsEl {}

impl BuildBigqueryReservationTimeoutsEl {
    pub fn build(self) -> BigqueryReservationTimeoutsEl {
        BigqueryReservationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct BigqueryReservationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryReservationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryReservationTimeoutsElRef {
        BigqueryReservationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryReservationTimeoutsElRef {
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
struct BigqueryReservationDynamic {
    autoscale: Option<DynamicBlock<BigqueryReservationAutoscaleEl>>,
}
