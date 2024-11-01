mod decoder;
mod encoder;
mod errors;
#[cfg(test)]
mod tests;
pub mod vault;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern "C" {
//     // Use `js_namespace` here to bind `console.log(..)` instead of just
//     // `log(..)`
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);

//     // The `console.log` is quite polymorphic, so we can bind it with multiple
//     // signatures. Note that we need to use `js_name` to ensure we always call
//     // `log` in JS.
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_u32(a: u32);

//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_u64(a: u64);

//     // Multiple arguments too!
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_many(a: &str, b: &str);
// }

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the Wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.
