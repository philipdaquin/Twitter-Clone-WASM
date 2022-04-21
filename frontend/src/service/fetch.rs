use { 
    wasm_bindgen::*,
    wasm_bindgen_futures::*,
    web_sys::*,
};


pub enum Method { 
    GET,
    POST,
    PATCH,
    OPTIONS,
    DELETE
}
pub async fn fetch(
    url: String, 
    method: String, 
    payload: Option<String>
) -> Result<JsValue, JsValue> { 
    let mut opts = RequestInit::new();
    opts.method(&method);
    opts.mode(RequestMode::Cors);
    opts.redirect(RequestRedirect::Follow);

    let req = Request::new_with_str_and_init(&url, &opts)?;
        req.headers().set("Accept", "application/json")?;
        req.headers().set("Content-Type", "application/json")?;
        req.headers().set("Acces-Control-Request-Header", "Conten-Type, Authorization")?;
        req.headers().set("Access-Control-Request-Method",  &method)?;
    let window = web_sys::window().unwrap();
    let resp_val = JsFuture::from(window.fetch_with_request(&req)).await?;

    let resp: Response = resp_val.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
} 
pub struct Fetch();

impl Fetch { 
    async fn fetch(url: String, method: Method, payload: Option<String>) -> Result<JsValue, JsValue> { 
        let method = match method { 
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PATCH => "PATCH",
            Method::OPTIONS => "OPTIONS",
            Method::DELETE => "DELETE"
        };
        fetch(url, method.to_string(), payload).await
    }
    pub async fn get(url: String) -> Result<JsValue, JsValue> { 
        Fetch::fetch(url, Method::GET, None).await
    }
    pub async fn patch(url: String, payload: Option<String>) -> Result<JsValue, JsValue> { 
        Fetch::fetch(url, Method::PATCH, payload).await
    }
    pub async fn post(url: String, payload: Option<String>) -> Result<JsValue, JsValue> { 
        Fetch::fetch(url, Method::POST, payload).await
    }
    pub async fn delete(url: String) -> Result<JsValue, JsValue> { 
        Fetch::fetch(url, Method::DELETE, None).await
    }
}