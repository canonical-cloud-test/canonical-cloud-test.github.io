#[ores_page(
    renderer = "mash",
    delivery = "ssr_only",
    render = "dynamic",
    title = "User detail",
    database = "read_only",
    data_sources("orm:user_read"),
    tags("canonical-cloud-test", "dynamic")
)]
pub async fn page() {}
