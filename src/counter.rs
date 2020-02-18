use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, RwLock,
};
use tracing_core::{
    field::{Field, Visit},
    span::{Attributes, Id, Record},
    Event, Metadata, Subscriber,
};
use tracing_subscriber::layer::{Context, Layer};

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
    file: String,
    line: u64,
}

impl CounterLogVisitor {
    fn target(&self) -> String {
        format!(
            "{}:{}",
            self.file
                .trim_start_matches("/home/sam/.cargo/registry/src/github.com-1ecc6299db9ec823"),
            self.line
        )
    }
}

impl Visit for CounterLogVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        match field.name() {
            "log.file" => self.file = value.to_string(),
            _ => {}
        }
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        match field.name() {
            "log.line" => self.line = value,
            _ => {}
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {}
}

impl<S> Layer<S> for Counter
where
    S: Subscriber,
{
    fn new_span(&self, _: &Attributes, _: &Id, _: Context<S>) {}

    fn on_record(&self, _: &Id, _: &Record<'_>, _: Context<S>) {}

    fn on_event(&self, event: &Event<'_>, _: Context<S>) {
        if event.metadata().target() == "log" {
            let mut visitor = CounterLogVisitor::default();
            event.record(&mut visitor);
            self.incr(visitor.target());
            // for f in event.fields() {
            //     self.incr(f);
            // }
        }
    }

    fn on_enter(&self, _: &Id, _: Context<S>) {}

    fn on_exit(&self, _: &Id, _: Context<S>) {}

    fn on_close(&self, _: Id, _: Context<S>) {}
}
