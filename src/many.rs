use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

pub fn encode_request(req: Request) -> Result<Request, JsValue> {
    let req_url = req.url();
    let req_body = make_request_bytes().unwrap();
    let req_init = make_request_init(req_body);

    let req = Request::new_with_str_and_init(&req_url, &req_init);
    req
}

pub fn decode_response(res: Response) -> Result<Response, JsValue> {
    // Don't do anything to the response
    Ok(res)
}

fn make_request_bytes() -> Result<JsValue, JsValue> {
    // Just send preformatted CBOR for now
    let bytes = "d28440a053d92711a2036673746174757305c11a64b81fa240".as_bytes();
    let bytes_value = serde_wasm_bindgen::to_value(bytes).unwrap();

    Ok(bytes_value)
}

fn make_request_init(req_body: JsValue) -> RequestInit {
    let headers = Headers::new().unwrap();
    headers.set("Content-Type", "application/cbor").unwrap();

    let body_buffer = Uint8Array::from(req_body);

    let mut req_init = RequestInit::new();
    req_init.headers(&headers);
    req_init.method("POST");
    req_init.mode(RequestMode::Cors);
    req_init.body(Some(&body_buffer));
    req_init
}
