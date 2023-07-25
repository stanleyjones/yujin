mod many;

use many::{decode_response, encode_request};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response, ServiceWorkerGlobalScope};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn refetch(req_value: JsValue) -> Result<JsValue, JsValue> {
    set_panic_hook();
    let scope = get_scope().unwrap();

    let req: Request = req_value.dyn_into().unwrap();
    let req = encode_request(req)?;

    log("[WASM] Fetching");
    let res_value = JsFuture::from(scope.fetch_with_request(&req)).await;
    match res_value {
        Ok(res_value) => {
            log("[WASM] Fetch success");
            let res: Response = res_value.dyn_into().unwrap();
            let res = decode_response(res)?;
            Ok(JsValue::from(res))
        }
        Err(err) => {
            log("[WASM] Fetch error");
            Err(err)
        }
    }
}

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn get_scope() -> Result<ServiceWorkerGlobalScope, JsValue> {
    let global = js_sys::global();
    Ok(global.dyn_into::<ServiceWorkerGlobalScope>()?)
}
