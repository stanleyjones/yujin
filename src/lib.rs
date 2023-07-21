use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response, ServiceWorkerGlobalScope};

#[derive(Serialize, Deserialize)]
struct ManyRequest {
    url: String,
    method: String,
    args: String,
}

#[wasm_bindgen]
pub async fn handle_request(req_value: JsValue) -> Result<JsValue, JsValue> {
    set_panic_hook();
    let scope = get_scope().unwrap();

    let incoming_req: ManyRequest = serde_wasm_bindgen::from_value(req_value)?;

    let req_url = incoming_req.url.as_str();
    let req_body = make_request_bytes()?;
    let req_init = make_request_init(req_body);
    let req = Request::new_with_str_and_init(req_url, &req_init)?;

    let res_value = JsFuture::from(scope.fetch_with_request(&req)).await?;
    let res: Response = res_value.dyn_into().unwrap();
    let array_buffer = JsFuture::from(res.array_buffer()?).await?;

    Ok(array_buffer)
}

fn make_request_bytes() -> Result<JsValue, serde_wasm_bindgen::Error> {
    // Just send preformatted CBOR for now
    serde_wasm_bindgen::to_value("d28440a053d92711a2036673746174757305c11a64b81fa240".as_bytes())
}

fn make_request_init(req_body: JsValue) -> RequestInit {
    let headers = Headers::new().unwrap();
    headers.set("Content-Type", "application/cbor").unwrap();

    let mut req_init = RequestInit::new();
    req_init.headers(&headers);
    req_init.method("POST");
    req_init.mode(RequestMode::Cors);
    req_init.body(Some(&req_body));
    req_init
}

fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

fn get_scope() -> Result<ServiceWorkerGlobalScope, JsValue> {
    let global = js_sys::global();
    Ok(global.dyn_into::<ServiceWorkerGlobalScope>()?)
}
