use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeRegionTargetHttpsProxyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    ssl_certificates: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_policy: Option<PrimField<String>>,
    url_map: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeRegionTargetHttpsProxyTimeoutsEl>,
}

struct ComputeRegionTargetHttpsProxy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeRegionTargetHttpsProxyData>,
}

#[derive(Clone)]
pub struct ComputeRegionTargetHttpsProxy(Rc<ComputeRegionTargetHttpsProxy_>);

impl ComputeRegionTargetHttpsProxy {
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

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\nThe Region in which the created target https proxy should reside.\nIf it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Set the field `ssl_policy`.\nA reference to the Region SslPolicy resource that will be associated with\nthe TargetHttpsProxy resource. If not set, the TargetHttpsProxy\nresource will not have any SSL policy configured."]
    pub fn set_ssl_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ssl_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeRegionTargetHttpsProxyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proxy_id` after provisioning.\nThe unique identifier for the resource."]
    pub fn proxy_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe Region in which the created target https proxy should reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssl_certificates` after provisioning.\nA list of RegionSslCertificate resources that are used to authenticate\nconnections between users and the load balancer. Currently, exactly\none SSL certificate must be specified."]
    pub fn ssl_certificates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ssl_certificates", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssl_policy` after provisioning.\nA reference to the Region SslPolicy resource that will be associated with\nthe TargetHttpsProxy resource. If not set, the TargetHttpsProxy\nresource will not have any SSL policy configured."]
    pub fn ssl_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url_map` after provisioning.\nA reference to the RegionUrlMap resource that defines the mapping from URL\nto the RegionBackendService."]
    pub fn url_map(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url_map", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionTargetHttpsProxyTimeoutsElRef {
        ComputeRegionTargetHttpsProxyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for ComputeRegionTargetHttpsProxy {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComputeRegionTargetHttpsProxy { }

impl ToListMappable for ComputeRegionTargetHttpsProxy {
    type O = ListRef<ComputeRegionTargetHttpsProxyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComputeRegionTargetHttpsProxy_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_region_target_https_proxy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeRegionTargetHttpsProxy {
    pub tf_id: String,
    #[doc= "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc= "A list of RegionSslCertificate resources that are used to authenticate\nconnections between users and the load balancer. Currently, exactly\none SSL certificate must be specified."]
    pub ssl_certificates: ListField<PrimField<String>>,
    #[doc= "A reference to the RegionUrlMap resource that defines the mapping from URL\nto the RegionBackendService."]
    pub url_map: PrimField<String>,
}

impl BuildComputeRegionTargetHttpsProxy {
    pub fn build(self, stack: &mut Stack) -> ComputeRegionTargetHttpsProxy {
        let out = ComputeRegionTargetHttpsProxy(Rc::new(ComputeRegionTargetHttpsProxy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeRegionTargetHttpsProxyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                ssl_certificates: self.ssl_certificates,
                ssl_policy: core::default::Default::default(),
                url_map: self.url_map,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeRegionTargetHttpsProxyRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionTargetHttpsProxyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeRegionTargetHttpsProxyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proxy_id` after provisioning.\nThe unique identifier for the resource."]
    pub fn proxy_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe Region in which the created target https proxy should reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssl_certificates` after provisioning.\nA list of RegionSslCertificate resources that are used to authenticate\nconnections between users and the load balancer. Currently, exactly\none SSL certificate must be specified."]
    pub fn ssl_certificates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ssl_certificates", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssl_policy` after provisioning.\nA reference to the Region SslPolicy resource that will be associated with\nthe TargetHttpsProxy resource. If not set, the TargetHttpsProxy\nresource will not have any SSL policy configured."]
    pub fn ssl_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url_map` after provisioning.\nA reference to the RegionUrlMap resource that defines the mapping from URL\nto the RegionBackendService."]
    pub fn url_map(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url_map", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionTargetHttpsProxyTimeoutsElRef {
        ComputeRegionTargetHttpsProxyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ComputeRegionTargetHttpsProxyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ComputeRegionTargetHttpsProxyTimeoutsEl {
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

impl ToListMappable for ComputeRegionTargetHttpsProxyTimeoutsEl {
    type O = BlockAssignable<ComputeRegionTargetHttpsProxyTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionTargetHttpsProxyTimeoutsEl {}

impl BuildComputeRegionTargetHttpsProxyTimeoutsEl {
    pub fn build(self) -> ComputeRegionTargetHttpsProxyTimeoutsEl {
        ComputeRegionTargetHttpsProxyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionTargetHttpsProxyTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionTargetHttpsProxyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionTargetHttpsProxyTimeoutsElRef {
        ComputeRegionTargetHttpsProxyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionTargetHttpsProxyTimeoutsElRef {
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
