#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeServiceAttachmentData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataComputeServiceAttachment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComputeServiceAttachmentData_>,
}

#[derive(Clone)]
pub struct DataComputeServiceAttachment(std::rc::Rc<DataComputeServiceAttachment_>);

impl DataComputeServiceAttachment {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `region`.\nURL of the region where the resource resides."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `connected_endpoints` after provisioning.\nAn array of the consumer forwarding rules connected to this service\nattachment."]
    pub fn connected_endpoints(&self) -> terrars::ListRef<DataComputeServiceAttachment_ConnectedEndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connected_endpoints", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connection_preference` after provisioning.\nThe connection preference to use for this service attachment. Valid\nvalues include \"ACCEPT_AUTOMATIC\", \"ACCEPT_MANUAL\"."]
    pub fn connection_preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_preference", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consumer_accept_lists` after provisioning.\nAn array of projects that are allowed to connect to this service\nattachment."]
    pub fn consumer_accept_lists(&self) -> terrars::SetRef<DataComputeServiceAttachment_ConsumerAcceptListsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.consumer_accept_lists", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consumer_reject_lists` after provisioning.\nAn array of projects that are not allowed to connect to this service\nattachment."]
    pub fn consumer_reject_lists(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.consumer_reject_lists", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain_names` after provisioning.\nIf specified, the domain name will be used during the integration between\nthe PSC connected endpoints and the Cloud DNS. For example, this is a\nvalid domain name: \"p.mycompany.com.\". Current max number of domain names\nsupported is 1."]
    pub fn domain_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain_names", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_proxy_protocol` after provisioning.\nIf true, enable the proxy protocol which is for supplying client TCP/IP\naddress data in TCP connections that traverse proxies on their way to\ndestination servers."]
    pub fn enable_proxy_protocol(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_proxy_protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. This field is used internally during\nupdates of this resource."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nat_ips_per_endpoint` after provisioning.\nThe number of NAT IPs allocated per connected endpoint."]
    pub fn nat_ips_per_endpoint(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nat_ips_per_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nat_subnets` after provisioning.\nAn array of subnets that is provided for NAT in this service attachment."]
    pub fn nat_subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.nat_subnets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `propagated_connection_limit` after provisioning.\nThe number of consumer spokes that connected Private Service Connect endpoints can be propagated to through Network Connectivity Center.\nThis limit lets the service producer limit how many propagated Private Service Connect connections can be established to this service attachment from a single consumer.\n\nIf the connection preference of the service attachment is ACCEPT_MANUAL, the limit applies to each project or network that is listed in the consumer accept list.\nIf the connection preference of the service attachment is ACCEPT_AUTOMATIC, the limit applies to each project that contains a connected endpoint.\n\nIf unspecified, the default propagated connection limit is 250. To explicitly send a zero value, set 'send_propagated_connection_limit_if_zero = true'."]
    pub fn propagated_connection_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagated_connection_limit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_service_attachment_id` after provisioning.\nAn 128-bit global unique ID of the PSC service attachment."]
    pub fn psc_service_attachment_id(&self) -> terrars::ListRef<DataComputeServiceAttachment_PscServiceAttachmentIdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_service_attachment_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconcile_connections` after provisioning.\nThis flag determines whether a consumer accept/reject list change can reconcile the statuses of existing ACCEPTED or REJECTED PSC endpoints.\n\nIf false, connection policy update will only affect existing PENDING PSC endpoints. Existing ACCEPTED/REJECTED endpoints will remain untouched regardless how the connection policy is modified .\nIf true, update will affect both PENDING and ACCEPTED/REJECTED PSC endpoints. For example, an ACCEPTED PSC endpoint will be moved to REJECTED if its project is added to the reject list."]
    pub fn reconcile_connections(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconcile_connections", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nURL of the region where the resource resides."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `send_propagated_connection_limit_if_zero` after provisioning.\nControls the behavior of propagated_connection_limit.\nWhen false, setting propagated_connection_limit to zero causes the provider to use to the API's default value.\nWhen true, the provider will set propagated_connection_limit to zero.\nDefaults to false."]
    pub fn send_propagated_connection_limit_if_zero(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_propagated_connection_limit_if_zero", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `show_nat_ips` after provisioning.\nNOTE: This field is temporarily non-functional due to an underlying API issue.\nAny value provided here will be ignored until the API issue is resolved, expected around 2026-03.\n[If true, show NAT IPs of all connected endpoints.]"]
    pub fn show_nat_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show_nat_ips", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_service` after provisioning.\nThe URL of a service serving the endpoint identified by this service attachment."]
    pub fn target_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_service", self.extract_ref()))
    }
}

impl terrars::Referable for DataComputeServiceAttachment {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComputeServiceAttachment { }

impl terrars::ToListMappable for DataComputeServiceAttachment {
    type O = terrars::ListRef<DataComputeServiceAttachmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComputeServiceAttachment_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_compute_service_attachment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeServiceAttachment {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataComputeServiceAttachment {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComputeServiceAttachment {
        let out = DataComputeServiceAttachment(std::rc::Rc::new(DataComputeServiceAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComputeServiceAttachmentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataComputeServiceAttachmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeServiceAttachmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeServiceAttachmentRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `connected_endpoints` after provisioning.\nAn array of the consumer forwarding rules connected to this service\nattachment."]
    pub fn connected_endpoints(&self) -> terrars::ListRef<DataComputeServiceAttachment_ConnectedEndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connected_endpoints", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connection_preference` after provisioning.\nThe connection preference to use for this service attachment. Valid\nvalues include \"ACCEPT_AUTOMATIC\", \"ACCEPT_MANUAL\"."]
    pub fn connection_preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_preference", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consumer_accept_lists` after provisioning.\nAn array of projects that are allowed to connect to this service\nattachment."]
    pub fn consumer_accept_lists(&self) -> terrars::SetRef<DataComputeServiceAttachment_ConsumerAcceptListsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.consumer_accept_lists", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consumer_reject_lists` after provisioning.\nAn array of projects that are not allowed to connect to this service\nattachment."]
    pub fn consumer_reject_lists(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.consumer_reject_lists", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain_names` after provisioning.\nIf specified, the domain name will be used during the integration between\nthe PSC connected endpoints and the Cloud DNS. For example, this is a\nvalid domain name: \"p.mycompany.com.\". Current max number of domain names\nsupported is 1."]
    pub fn domain_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain_names", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_proxy_protocol` after provisioning.\nIf true, enable the proxy protocol which is for supplying client TCP/IP\naddress data in TCP connections that traverse proxies on their way to\ndestination servers."]
    pub fn enable_proxy_protocol(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_proxy_protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. This field is used internally during\nupdates of this resource."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nat_ips_per_endpoint` after provisioning.\nThe number of NAT IPs allocated per connected endpoint."]
    pub fn nat_ips_per_endpoint(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nat_ips_per_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nat_subnets` after provisioning.\nAn array of subnets that is provided for NAT in this service attachment."]
    pub fn nat_subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.nat_subnets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `propagated_connection_limit` after provisioning.\nThe number of consumer spokes that connected Private Service Connect endpoints can be propagated to through Network Connectivity Center.\nThis limit lets the service producer limit how many propagated Private Service Connect connections can be established to this service attachment from a single consumer.\n\nIf the connection preference of the service attachment is ACCEPT_MANUAL, the limit applies to each project or network that is listed in the consumer accept list.\nIf the connection preference of the service attachment is ACCEPT_AUTOMATIC, the limit applies to each project that contains a connected endpoint.\n\nIf unspecified, the default propagated connection limit is 250. To explicitly send a zero value, set 'send_propagated_connection_limit_if_zero = true'."]
    pub fn propagated_connection_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagated_connection_limit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_service_attachment_id` after provisioning.\nAn 128-bit global unique ID of the PSC service attachment."]
    pub fn psc_service_attachment_id(&self) -> terrars::ListRef<DataComputeServiceAttachment_PscServiceAttachmentIdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_service_attachment_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconcile_connections` after provisioning.\nThis flag determines whether a consumer accept/reject list change can reconcile the statuses of existing ACCEPTED or REJECTED PSC endpoints.\n\nIf false, connection policy update will only affect existing PENDING PSC endpoints. Existing ACCEPTED/REJECTED endpoints will remain untouched regardless how the connection policy is modified .\nIf true, update will affect both PENDING and ACCEPTED/REJECTED PSC endpoints. For example, an ACCEPTED PSC endpoint will be moved to REJECTED if its project is added to the reject list."]
    pub fn reconcile_connections(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconcile_connections", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nURL of the region where the resource resides."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `send_propagated_connection_limit_if_zero` after provisioning.\nControls the behavior of propagated_connection_limit.\nWhen false, setting propagated_connection_limit to zero causes the provider to use to the API's default value.\nWhen true, the provider will set propagated_connection_limit to zero.\nDefaults to false."]
    pub fn send_propagated_connection_limit_if_zero(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_propagated_connection_limit_if_zero", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `show_nat_ips` after provisioning.\nNOTE: This field is temporarily non-functional due to an underlying API issue.\nAny value provided here will be ignored until the API issue is resolved, expected around 2026-03.\n[If true, show NAT IPs of all connected endpoints.]"]
    pub fn show_nat_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show_nat_ips", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_service` after provisioning.\nThe URL of a service serving the endpoint identified by this service attachment."]
    pub fn target_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_service", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComputeServiceAttachment_ConnectedEndpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_ips: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagated_connection_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeServiceAttachment_ConnectedEndpoints {
    #[doc = "Set the field `consumer_network`.\n"]
    pub fn set_consumer_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consumer_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nat_ips`.\n"]
    pub fn set_nat_ips(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.nat_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `propagated_connection_count`.\n"]
    pub fn set_propagated_connection_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.propagated_connection_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_connection_id`.\n"]
    pub fn set_psc_connection_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.psc_connection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeServiceAttachment_ConnectedEndpoints {
    type O = terrars::BlockAssignable<DataComputeServiceAttachment_ConnectedEndpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeServiceAttachment_ConnectedEndpoints {}

impl BuildDataComputeServiceAttachment_ConnectedEndpoints {
    pub fn build(self) -> DataComputeServiceAttachment_ConnectedEndpoints {
        DataComputeServiceAttachment_ConnectedEndpoints {
            consumer_network: core::default::Default::default(),
            endpoint: core::default::Default::default(),
            nat_ips: core::default::Default::default(),
            propagated_connection_count: core::default::Default::default(),
            psc_connection_id: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeServiceAttachment_ConnectedEndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeServiceAttachment_ConnectedEndpointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeServiceAttachment_ConnectedEndpointsRef {
        DataComputeServiceAttachment_ConnectedEndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeServiceAttachment_ConnectedEndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `consumer_network` after provisioning.\n"]
    pub fn consumer_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_network", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `nat_ips` after provisioning.\n"]
    pub fn nat_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nat_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `propagated_connection_count` after provisioning.\n"]
    pub fn propagated_connection_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagated_connection_count", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_connection_id` after provisioning.\n"]
    pub fn psc_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_connection_id", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeServiceAttachment_ConsumerAcceptLists {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id_or_num: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeServiceAttachment_ConsumerAcceptLists {
    #[doc = "Set the field `connection_limit`.\n"]
    pub fn set_connection_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.connection_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_url`.\n"]
    pub fn set_endpoint_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_url`.\n"]
    pub fn set_network_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id_or_num`.\n"]
    pub fn set_project_id_or_num(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id_or_num = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeServiceAttachment_ConsumerAcceptLists {
    type O = terrars::BlockAssignable<DataComputeServiceAttachment_ConsumerAcceptLists>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeServiceAttachment_ConsumerAcceptLists {}

impl BuildDataComputeServiceAttachment_ConsumerAcceptLists {
    pub fn build(self) -> DataComputeServiceAttachment_ConsumerAcceptLists {
        DataComputeServiceAttachment_ConsumerAcceptLists {
            connection_limit: core::default::Default::default(),
            endpoint_url: core::default::Default::default(),
            network_url: core::default::Default::default(),
            project_id_or_num: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeServiceAttachment_ConsumerAcceptListsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeServiceAttachment_ConsumerAcceptListsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeServiceAttachment_ConsumerAcceptListsRef {
        DataComputeServiceAttachment_ConsumerAcceptListsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeServiceAttachment_ConsumerAcceptListsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_limit` after provisioning.\n"]
    pub fn connection_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_url` after provisioning.\n"]
    pub fn endpoint_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_url", self.base))
    }

    #[doc = "Get a reference to the value of field `network_url` after provisioning.\n"]
    pub fn network_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_url", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id_or_num` after provisioning.\n"]
    pub fn project_id_or_num(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id_or_num", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeServiceAttachment_PscServiceAttachmentId {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    high: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    low: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeServiceAttachment_PscServiceAttachmentId {
    #[doc = "Set the field `high`.\n"]
    pub fn set_high(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.high = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `low`.\n"]
    pub fn set_low(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.low = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeServiceAttachment_PscServiceAttachmentId {
    type O = terrars::BlockAssignable<DataComputeServiceAttachment_PscServiceAttachmentId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeServiceAttachment_PscServiceAttachmentId {}

impl BuildDataComputeServiceAttachment_PscServiceAttachmentId {
    pub fn build(self) -> DataComputeServiceAttachment_PscServiceAttachmentId {
        DataComputeServiceAttachment_PscServiceAttachmentId {
            high: core::default::Default::default(),
            low: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeServiceAttachment_PscServiceAttachmentIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeServiceAttachment_PscServiceAttachmentIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeServiceAttachment_PscServiceAttachmentIdRef {
        DataComputeServiceAttachment_PscServiceAttachmentIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeServiceAttachment_PscServiceAttachmentIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `high` after provisioning.\n"]
    pub fn high(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.high", self.base))
    }

    #[doc = "Get a reference to the value of field `low` after provisioning.\n"]
    pub fn low(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.low", self.base))
    }
}
