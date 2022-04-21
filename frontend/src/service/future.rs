use std::future::Future;
use wasm_bindgen_futures::spawn_local;

pub fn handle_future<A, B, C>(future: A, handler: B) 
where 
    A: Future<Output = C> + 'static, B: Fn(C) + 'static,
    { 
        spawn_local(async move { 
            let res: C = future.await;
            handler(res)
        });
    }