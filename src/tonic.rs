use http::HeaderMap;
use lazy_static::lazy_static;
use opentelemetry::api::{self, HttpTextFormat};
use opentelemetry_datadog::propagation::DatadogPropagator;
use tracing::{Span, info_span};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tonic::{Request, Status, metadata::MetadataMap};

lazy_static! {
    static ref DATADOG_PROPAGATOR: DatadogPropagator = DatadogPropagator::new();
}

struct HttpHeaderMapCarrier<'a>(&'a HeaderMap);

impl<'a> From<&'a HeaderMap> for HttpHeaderMapCarrier<'a> {
    fn from(header_map: &'a HeaderMap) -> Self {
        HttpHeaderMapCarrier(header_map)
    }
}

impl<'a> api::Carrier for HttpHeaderMapCarrier<'a> {
    fn get(&self, key: &'static str) -> Option<&str> {
        self.0
            .get(key.to_lowercase().as_str())
            .and_then(|value| value.to_str().ok())
    }

    fn set(&mut self, _key: &'static str, _value: String) {
        unimplemented!()
    }
}

struct MetadataMapCarrier<'a>(&'a mut MetadataMap);

impl<'a> From<&'a mut MetadataMap> for MetadataMapCarrier<'a> {
    fn from(metadata_map: &'a mut MetadataMap) -> Self {
        MetadataMapCarrier(metadata_map)
    }
}

impl<'a> api::Carrier for MetadataMapCarrier<'a> {
    fn get(&self, key: &'static str) -> Option<&str> {
        unimplemented!()
    }

    fn set(&mut self, key: &'static str, value: String) {
        self.0.insert(key, value.parse().unwrap());
    }
}

pub fn extract_span(header: &HeaderMap) -> tracing::Span {
    let parent = DATADOG_PROPAGATOR.extract(&HttpHeaderMapCarrier::from(header));
    let tracing_span = info_span!("grpc.request");
    if parent.is_valid() {
        tracing_span.set_parent(parent);
    }
    tracing_span
}

pub fn intercept(mut req: Request<()>) -> Result<Request<()>, Status> {
    let context = Span::current().context();
    DATADOG_PROPAGATOR.inject(context, &mut MetadataMapCarrier::from(req.metadata_mut()));
    Ok(req)
}
