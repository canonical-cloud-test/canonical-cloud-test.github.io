#[ores_page(
    renderer = "mash",
    delivery = "ssr_only",
    render = "dynamic",
    title = "Canonical test home",
    tags("canonical-cloud-test", "ores-stack")
)]
pub async fn page() {}
