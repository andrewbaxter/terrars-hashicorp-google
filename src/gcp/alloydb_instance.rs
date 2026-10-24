use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AlloydbInstanceData {
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
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_type: std::option::Option<terrars::PrimField<std::string::String>>,
    cluster: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_flags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gce_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_id: terrars::PrimField<std::string::String>,
    instance_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_connection_config: std::option::Option<std::vec::Vec<AlloydbInstanceClientConnectionConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_config: std::option::Option<std::vec::Vec<AlloydbInstanceMachineConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_insights_config: std::option::Option<std::vec::Vec<AlloydbInstanceQueryInsightsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_pool_config: std::option::Option<std::vec::Vec<AlloydbInstanceReadPoolConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AlloydbInstanceTimeoutsEl>,
    dynamic: AlloydbInstanceDynamic,
}

struct AlloydbInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AlloydbInstanceData>,
}

#[derive(Clone)]
pub struct AlloydbInstance(std::rc::Rc<AlloydbInstance_>);

impl AlloydbInstance {
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
        "Set the field `annotations`.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `availability_type`.\n'Availability type of an Instance. Defaults to REGIONAL for both primary and read instances.\nNote that primary and read instances can have different availability types.\nOnly READ_POOL instance supports ZONAL type. Users can't specify the zone for READ_POOL instance.\nZone is automatically chosen from the list of zones in the region specified.\nRead pool of size 1 can only have zonal availability. Read pools with node count of 2 or more\ncan have regional availability (nodes are present in 2 or more zones in a region).' Possible values: [\"AVAILABILITY_TYPE_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub fn set_availability_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().availability_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `database_flags`.\nDatabase flags. Set at instance level. * They are copied from primary instance on read instance creation. * Read instances can set new or override existing flags that are relevant for reads, e.g. for enabling columnar cache on a read instance. Flags set on read instance may or may not be present on primary."]
    pub fn set_database_flags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().database_flags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nUser-settable and human-readable display name for the Instance."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gce_zone`.\nThe Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity."]
    pub fn set_gce_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().gce_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUser-defined labels for the alloydb instance.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_connection_config`.\n"]
    pub fn set_client_connection_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbInstanceClientConnectionConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_connection_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_connection_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `machine_config`.\n"]
    pub fn set_machine_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbInstanceMachineConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().machine_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.machine_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_insights_config`.\n"]
    pub fn set_query_insights_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbInstanceQueryInsightsConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().query_insights_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.query_insights_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `read_pool_config`.\n"]
    pub fn set_read_pool_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbInstanceReadPoolConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().read_pool_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.read_pool_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AlloydbInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_type` after provisioning.\n'Availability type of an Instance. Defaults to REGIONAL for both primary and read instances.\nNote that primary and read instances can have different availability types.\nOnly READ_POOL instance supports ZONAL type. Users can't specify the zone for READ_POOL instance.\nZone is automatically chosen from the list of zones in the region specified.\nRead pool of size 1 can only have zonal availability. Read pools with node count of 2 or more\ncan have regional availability (nodes are present in 2 or more zones in a region).' Possible values: [\"AVAILABILITY_TYPE_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub fn availability_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nIdentifies the alloydb cluster. Must be in the format\n'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the Instance was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_flags` after provisioning.\nDatabase flags. Set at instance level. * They are copied from primary instance on read instance creation. * Read instances can set new or override existing flags that are relevant for reads, e.g. for enabling columnar cache on a read instance. Flags set on read instance may or may not be present on primary."]
    pub fn database_flags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.database_flags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-settable and human-readable display name for the Instance."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gce_zone` after provisioning.\nThe Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity."]
    pub fn gce_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gce_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe ID of the alloydb instance."]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_type` after provisioning.\nThe type of the instance.\nIf the instance type is READ_POOL, provide the associated PRIMARY/SECONDARY instance in the 'depends_on' meta-data attribute.\nIf the instance type is SECONDARY, point to the cluster_type of the associated secondary cluster instead of mentioning SECONDARY.\nExample: {instance_type = google_alloydb_cluster.<secondary_cluster_name>.cluster_type} instead of {instance_type = SECONDARY}\nIf the instance type is SECONDARY, the terraform delete instance operation does not delete the secondary instance but abandons it instead.\nUse deletion_policy = \"FORCE\" in the associated secondary cluster and delete the cluster forcefully to delete the secondary cluster as well its associated secondary instance.\nUsers can undo the delete secondary instance action by importing the deleted secondary instance by calling terraform import. Possible values: [\"PRIMARY\", \"READ_POOL\", \"SECONDARY\"]"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address for the Instance. This is the connection endpoint for an end-user application."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the alloydb instance.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the instance resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nSet to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the alloydb instance."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the Instance was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_connection_config` after provisioning.\n"]
    pub fn client_connection_config(&self) -> terrars::ListRef<AlloydbInstanceClientConnectionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_connection_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `machine_config` after provisioning.\n"]
    pub fn machine_config(&self) -> terrars::ListRef<AlloydbInstanceMachineConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_insights_config` after provisioning.\n"]
    pub fn query_insights_config(&self) -> terrars::ListRef<AlloydbInstanceQueryInsightsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_insights_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_pool_config` after provisioning.\n"]
    pub fn read_pool_config(&self) -> terrars::ListRef<AlloydbInstanceReadPoolConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_pool_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AlloydbInstanceTimeoutsElRef {
        AlloydbInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for AlloydbInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AlloydbInstance { }

impl terrars::ToListMappable for AlloydbInstance {
    type O = terrars::ListRef<AlloydbInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AlloydbInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_alloydb_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAlloydbInstance {
    pub tf_id: std::string::String,
    #[doc =
        "Identifies the alloydb cluster. Must be in the format\n'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub cluster: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the alloydb instance."]
    pub instance_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of the instance.\nIf the instance type is READ_POOL, provide the associated PRIMARY/SECONDARY instance in the 'depends_on' meta-data attribute.\nIf the instance type is SECONDARY, point to the cluster_type of the associated secondary cluster instead of mentioning SECONDARY.\nExample: {instance_type = google_alloydb_cluster.<secondary_cluster_name>.cluster_type} instead of {instance_type = SECONDARY}\nIf the instance type is SECONDARY, the terraform delete instance operation does not delete the secondary instance but abandons it instead.\nUse deletion_policy = \"FORCE\" in the associated secondary cluster and delete the cluster forcefully to delete the secondary cluster as well its associated secondary instance.\nUsers can undo the delete secondary instance action by importing the deleted secondary instance by calling terraform import. Possible values: [\"PRIMARY\", \"READ_POOL\", \"SECONDARY\"]"]
    pub instance_type: terrars::PrimField<std::string::String>,
}

impl BuildAlloydbInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> AlloydbInstance {
        let out = AlloydbInstance(std::rc::Rc::new(AlloydbInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AlloydbInstanceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                availability_type: core::default::Default::default(),
                cluster: self.cluster,
                database_flags: core::default::Default::default(),
                display_name: core::default::Default::default(),
                gce_zone: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                instance_type: self.instance_type,
                labels: core::default::Default::default(),
                client_connection_config: core::default::Default::default(),
                machine_config: core::default::Default::default(),
                query_insights_config: core::default::Default::default(),
                read_pool_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AlloydbInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AlloydbInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_type` after provisioning.\n'Availability type of an Instance. Defaults to REGIONAL for both primary and read instances.\nNote that primary and read instances can have different availability types.\nOnly READ_POOL instance supports ZONAL type. Users can't specify the zone for READ_POOL instance.\nZone is automatically chosen from the list of zones in the region specified.\nRead pool of size 1 can only have zonal availability. Read pools with node count of 2 or more\ncan have regional availability (nodes are present in 2 or more zones in a region).' Possible values: [\"AVAILABILITY_TYPE_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub fn availability_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nIdentifies the alloydb cluster. Must be in the format\n'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the Instance was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_flags` after provisioning.\nDatabase flags. Set at instance level. * They are copied from primary instance on read instance creation. * Read instances can set new or override existing flags that are relevant for reads, e.g. for enabling columnar cache on a read instance. Flags set on read instance may or may not be present on primary."]
    pub fn database_flags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.database_flags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-settable and human-readable display name for the Instance."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gce_zone` after provisioning.\nThe Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity."]
    pub fn gce_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gce_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe ID of the alloydb instance."]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_type` after provisioning.\nThe type of the instance.\nIf the instance type is READ_POOL, provide the associated PRIMARY/SECONDARY instance in the 'depends_on' meta-data attribute.\nIf the instance type is SECONDARY, point to the cluster_type of the associated secondary cluster instead of mentioning SECONDARY.\nExample: {instance_type = google_alloydb_cluster.<secondary_cluster_name>.cluster_type} instead of {instance_type = SECONDARY}\nIf the instance type is SECONDARY, the terraform delete instance operation does not delete the secondary instance but abandons it instead.\nUse deletion_policy = \"FORCE\" in the associated secondary cluster and delete the cluster forcefully to delete the secondary cluster as well its associated secondary instance.\nUsers can undo the delete secondary instance action by importing the deleted secondary instance by calling terraform import. Possible values: [\"PRIMARY\", \"READ_POOL\", \"SECONDARY\"]"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address for the Instance. This is the connection endpoint for an end-user application."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the alloydb instance.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the instance resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nSet to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the alloydb instance."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the Instance was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_connection_config` after provisioning.\n"]
    pub fn client_connection_config(&self) -> terrars::ListRef<AlloydbInstanceClientConnectionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_connection_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `machine_config` after provisioning.\n"]
    pub fn machine_config(&self) -> terrars::ListRef<AlloydbInstanceMachineConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_insights_config` after provisioning.\n"]
    pub fn query_insights_config(&self) -> terrars::ListRef<AlloydbInstanceQueryInsightsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_insights_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_pool_config` after provisioning.\n"]
    pub fn read_pool_config(&self) -> terrars::ListRef<AlloydbInstanceReadPoolConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_pool_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AlloydbInstanceTimeoutsElRef {
        AlloydbInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AlloydbInstanceClientConnectionConfigElSslConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbInstanceClientConnectionConfigElSslConfigEl {
    #[doc =
        "Set the field `ssl_mode`.\nSSL mode. Specifies client-server SSL/TLS connection behavior. Possible values: [\"ENCRYPTED_ONLY\", \"ALLOW_UNENCRYPTED_AND_ENCRYPTED\"]"]
    pub fn set_ssl_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbInstanceClientConnectionConfigElSslConfigEl {
    type O = terrars::BlockAssignable<AlloydbInstanceClientConnectionConfigElSslConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbInstanceClientConnectionConfigElSslConfigEl {}

impl BuildAlloydbInstanceClientConnectionConfigElSslConfigEl {
    pub fn build(self) -> AlloydbInstanceClientConnectionConfigElSslConfigEl {
        AlloydbInstanceClientConnectionConfigElSslConfigEl { ssl_mode: core::default::Default::default() }
    }
}

pub struct AlloydbInstanceClientConnectionConfigElSslConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstanceClientConnectionConfigElSslConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbInstanceClientConnectionConfigElSslConfigElRef {
        AlloydbInstanceClientConnectionConfigElSslConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbInstanceClientConnectionConfigElSslConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ssl_mode` after provisioning.\nSSL mode. Specifies client-server SSL/TLS connection behavior. Possible values: [\"ENCRYPTED_ONLY\", \"ALLOW_UNENCRYPTED_AND_ENCRYPTED\"]"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlloydbInstanceClientConnectionConfigElDynamic {
    ssl_config: std::option::Option<terrars::DynamicBlock<AlloydbInstanceClientConnectionConfigElSslConfigEl>>,
}

#[derive(Serialize)]
pub struct AlloydbInstanceClientConnectionConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_connectors: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_config: std::option::Option<std::vec::Vec<AlloydbInstanceClientConnectionConfigElSslConfigEl>>,
    dynamic: AlloydbInstanceClientConnectionConfigElDynamic,
}

impl AlloydbInstanceClientConnectionConfigEl {
    #[doc =
        "Set the field `require_connectors`.\nConfiguration to enforce connectors only (ex: AuthProxy) connections to the database."]
    pub fn set_require_connectors(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_connectors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_config`.\n"]
    pub fn set_ssl_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbInstanceClientConnectionConfigElSslConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssl_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssl_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AlloydbInstanceClientConnectionConfigEl {
    type O = terrars::BlockAssignable<AlloydbInstanceClientConnectionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbInstanceClientConnectionConfigEl {}

impl BuildAlloydbInstanceClientConnectionConfigEl {
    pub fn build(self) -> AlloydbInstanceClientConnectionConfigEl {
        AlloydbInstanceClientConnectionConfigEl {
            require_connectors: core::default::Default::default(),
            ssl_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AlloydbInstanceClientConnectionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstanceClientConnectionConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbInstanceClientConnectionConfigElRef {
        AlloydbInstanceClientConnectionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbInstanceClientConnectionConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `require_connectors` after provisioning.\nConfiguration to enforce connectors only (ex: AuthProxy) connections to the database."]
    pub fn require_connectors(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_connectors", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_config` after provisioning.\n"]
    pub fn ssl_config(&self) -> terrars::ListRef<AlloydbInstanceClientConnectionConfigElSslConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbInstanceMachineConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_count: std::option::Option<terrars::PrimField<f64>>,
}

impl AlloydbInstanceMachineConfigEl {
    #[doc = "Set the field `cpu_count`.\nThe number of CPU's in the VM instance."]
    pub fn set_cpu_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbInstanceMachineConfigEl {
    type O = terrars::BlockAssignable<AlloydbInstanceMachineConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbInstanceMachineConfigEl {}

impl BuildAlloydbInstanceMachineConfigEl {
    pub fn build(self) -> AlloydbInstanceMachineConfigEl {
        AlloydbInstanceMachineConfigEl { cpu_count: core::default::Default::default() }
    }
}

pub struct AlloydbInstanceMachineConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstanceMachineConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbInstanceMachineConfigElRef {
        AlloydbInstanceMachineConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbInstanceMachineConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpu_count` after provisioning.\nThe number of CPU's in the VM instance."]
    pub fn cpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbInstanceQueryInsightsConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_plans_per_minute: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_application_tags: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_client_address: std::option::Option<terrars::PrimField<bool>>,
}

impl AlloydbInstanceQueryInsightsConfigEl {
    #[doc =
        "Set the field `query_plans_per_minute`.\nNumber of query execution plans captured by Insights per minute for all queries combined. The default value is 5. Any integer between 0 and 20 is considered valid."]
    pub fn set_query_plans_per_minute(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.query_plans_per_minute = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_string_length`.\nQuery string length. The default value is 1024. Any integer between 256 and 4500 is considered valid."]
    pub fn set_query_string_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.query_string_length = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `record_application_tags`.\nRecord application tags for an instance. This flag is turned \"on\" by default."]
    pub fn set_record_application_tags(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.record_application_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `record_client_address`.\nRecord client address for an instance. Client address is PII information. This flag is turned \"on\" by default."]
    pub fn set_record_client_address(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.record_client_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbInstanceQueryInsightsConfigEl {
    type O = terrars::BlockAssignable<AlloydbInstanceQueryInsightsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbInstanceQueryInsightsConfigEl {}

impl BuildAlloydbInstanceQueryInsightsConfigEl {
    pub fn build(self) -> AlloydbInstanceQueryInsightsConfigEl {
        AlloydbInstanceQueryInsightsConfigEl {
            query_plans_per_minute: core::default::Default::default(),
            query_string_length: core::default::Default::default(),
            record_application_tags: core::default::Default::default(),
            record_client_address: core::default::Default::default(),
        }
    }
}

pub struct AlloydbInstanceQueryInsightsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstanceQueryInsightsConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbInstanceQueryInsightsConfigElRef {
        AlloydbInstanceQueryInsightsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbInstanceQueryInsightsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `query_plans_per_minute` after provisioning.\nNumber of query execution plans captured by Insights per minute for all queries combined. The default value is 5. Any integer between 0 and 20 is considered valid."]
    pub fn query_plans_per_minute(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_plans_per_minute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_string_length` after provisioning.\nQuery string length. The default value is 1024. Any integer between 256 and 4500 is considered valid."]
    pub fn query_string_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_string_length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `record_application_tags` after provisioning.\nRecord application tags for an instance. This flag is turned \"on\" by default."]
    pub fn record_application_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_application_tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `record_client_address` after provisioning.\nRecord client address for an instance. Client address is PII information. This flag is turned \"on\" by default."]
    pub fn record_client_address(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_client_address", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbInstanceReadPoolConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
}

impl AlloydbInstanceReadPoolConfigEl {
    #[doc = "Set the field `node_count`.\nRead capacity, i.e. number of nodes in a read pool instance."]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbInstanceReadPoolConfigEl {
    type O = terrars::BlockAssignable<AlloydbInstanceReadPoolConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbInstanceReadPoolConfigEl {}

impl BuildAlloydbInstanceReadPoolConfigEl {
    pub fn build(self) -> AlloydbInstanceReadPoolConfigEl {
        AlloydbInstanceReadPoolConfigEl { node_count: core::default::Default::default() }
    }
}

pub struct AlloydbInstanceReadPoolConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstanceReadPoolConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbInstanceReadPoolConfigElRef {
        AlloydbInstanceReadPoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbInstanceReadPoolConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nRead capacity, i.e. number of nodes in a read pool instance."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbInstanceTimeoutsEl {
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

impl terrars::ToListMappable for AlloydbInstanceTimeoutsEl {
    type O = terrars::BlockAssignable<AlloydbInstanceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbInstanceTimeoutsEl {}

impl BuildAlloydbInstanceTimeoutsEl {
    pub fn build(self) -> AlloydbInstanceTimeoutsEl {
        AlloydbInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct AlloydbInstanceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstanceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbInstanceTimeoutsElRef {
        AlloydbInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbInstanceTimeoutsElRef {
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
struct AlloydbInstanceDynamic {
    client_connection_config: std::option::Option<terrars::DynamicBlock<AlloydbInstanceClientConnectionConfigEl>>,
    machine_config: std::option::Option<terrars::DynamicBlock<AlloydbInstanceMachineConfigEl>>,
    query_insights_config: std::option::Option<terrars::DynamicBlock<AlloydbInstanceQueryInsightsConfigEl>>,
    read_pool_config: std::option::Option<terrars::DynamicBlock<AlloydbInstanceReadPoolConfigEl>>,
}
