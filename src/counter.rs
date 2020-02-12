use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, RwLock,
};
use tracing_core::{
    span::{Attributes, Id, Record},
    Event, Metadata, Subscriber,
    field::{Field, Visit},
};
use tracing_subscriber::{
    layer::{Context, Layer},
};

#[derive(Default, Debug)]
pub struct Counter {
    pub counts: Arc<RwLock<HashMap<String, AtomicUsize>>>,
}

impl Counter {
    pub fn incr<K: AsRef<str> + ToString>(&self, key: K) {
        {
            let h = self.counts.read().unwrap();
            if let Some(count) = h.get(key.as_ref()) {
                count.fetch_add(1, Ordering::Relaxed);
                return;
            }
        }
        {
            let mut h = self.counts.write().unwrap();
            let count = h.entry(key.to_string()).or_insert(AtomicUsize::new(0));
            count.fetch_add(1, Ordering::Relaxed);
        }
    }
}

#[derive(Default)]
struct CounterLogVisitor {
    target: Option<String>,
}

impl CounterLogVisitor {
    fn target(self) -> String {
        self.target.unwrap_or("_unknown_target".to_string())
    }
}

impl Visit for CounterLogVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "log.target" {
            self.target = Some(value.to_string());
        }
    }

    fn record_debug(&mut self, _: &Field, _: &dyn fmt::Debug) {}
}

impl<S> Layer<S> for Counter
where
    S: Subscriber
{
    fn new_span(&self, _: &Attributes, _: &Id, _: Context<S>) {}

    fn on_record(&self, _: &Id, _: &Record<'_>, _: Context<S>) {}

    fn on_event(&self, event: &Event<'_>, _: Context<S>) {
        if event.metadata().target() == "log" {
            let mut visitor = CounterLogVisitor::default();
            event.record(&mut visitor);
            self.incr(visitor.target());
        }
    }

    fn on_enter(&self, _: &Id, _: Context<S>) {}

    fn on_exit(&self, _: &Id, _: Context<S>) {}

    fn on_close(&self, _: Id, _: Context<S>) {}
}
