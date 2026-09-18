#[ores_page(
    renderer = "mash",
    delivery = "ssr_only",
    render = "dynamic",
    title = "New user",
    tags("canonical-cloud-test", "static-precedence")
)]
pub async fn page() {}
