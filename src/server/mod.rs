mod srv_ctx;

// re-export server context
pub use srv_ctx::SrvCtx;

#[allow(dead_code)]
pub fn server(ctx: SrvCtx) {
    println!("server");
}
