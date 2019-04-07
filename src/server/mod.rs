#[derive(Debug)]
pub struct SrvCtx {
    port: u32,
}

#[allow(dead_code)]
pub fn server(ctx: SrvCtx) {
    println!("server");
}
