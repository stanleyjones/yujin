mod many;

use many::{decode_response, encode_request};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response, ServiceWorkerGlobalScope};

#[wasm_bindgen]
pub async fn refetch(req_value: JsValue) -> Result<JsValue, JsValue> {
    set_panic_hook();
    let scope = get_scope().unwrap();

    let req: Request = req_value.dyn_into().unwrap();
    let many_req = encode_request(req)?;

    let res_value = JsFuture::from(scope.fetch_with_request(&many_req)).await?;

    let many_res: Response = res_value.dyn_into().unwrap();
    let res = decode_response(many_res);

    Ok(JsValue::from(res))
}

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn get_scope() -> Result<ServiceWorkerGlobalScope, JsValue> {
    let global = js_sys::global();
    Ok(global.dyn_into::<ServiceWorkerGlobalScope>()?)
}
