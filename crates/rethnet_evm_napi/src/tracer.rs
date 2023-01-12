mod js_tracer;

use napi::Env;
use napi_derive::napi;
use rethnet_evm::{AsyncDatabase, Inspector};

use self::js_tracer::{JsTracer, TracingCallbacks};

#[napi]
pub struct Tracer {
    inner: Box<JsTracer>,
}

impl Tracer {
    pub fn as_dyn_inspector(
        &self,
    ) -> Box<(dyn Inspector<AsyncDatabase<anyhow::Error>> + Send + 'static)> {
        self.inner.clone()
    }
}

#[napi]
impl Tracer {
    #[napi(constructor)]
    pub fn new(env: Env, callbacks: TracingCallbacks) -> napi::Result<Self> {
        JsTracer::new(&env, callbacks).map(|inner| Self {
            inner: Box::new(inner),
        })
    }
}
