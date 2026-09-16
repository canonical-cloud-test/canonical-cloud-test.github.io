use std::{env, fs, path::PathBuf, process};

use ores_api_docs::{
    materialize_finalized_page_build, read_page_build_manifest, write_page_build_outputs,
};

fn main() {
    rejects_orm_without_read_only();
    rejects_write_capable_database();
    rejects_dynamic_static_only_without_generator();
    rejects_ambiguous_dynamic_siblings();
    rejects_unfinalized_browser_assets();
    println!("canonical-cloud-test ores-stack fail-closed certification passed");
}

fn rejects_orm_without_read_only() {
    let root = fresh_root("orm-without-read-only");
    write_page(
        &root,
        "src/pages/evidence/page.rs",
        r#"#[ores_page(
renderer = "mash",
delivery = "ssr_only",
data_sources("orm:evidence_read")
)]
pub async fn page() {}
"#,
    );
    let error = write_page_build_outputs(&root, &root.join(".ores-stack/first-pass"))
        .expect_err("ORM-backed page without read_only database must fail");
    let message = error.to_string().to_lowercase();
    assert!(message.contains("orm"), "{message}");
    assert!(message.contains("read_only") || message.contains("read-only"), "{message}");
    cleanup(root);
}

fn rejects_write_capable_database() {
    let root = fresh_root("write-capable-database");
    write_page(
        &root,
        "src/pages/admin/page.rs",
        r#"#[ores_page(
renderer = "mash",
delivery = "ssr_only",
database = "read_write"
)]
pub async fn page() {}
"#,
    );
    let error = write_page_build_outputs(&root, &root.join(".ores-stack/first-pass"))
        .expect_err("write-capable page database metadata must fail");
    let message = error.to_string().to_lowercase();
    assert!(message.contains("database"), "{message}");
    assert!(message.contains("read_only") || message.contains("read-only"), "{message}");
    cleanup(root);
}

fn rejects_dynamic_static_only_without_generator() {
    let root = fresh_root("missing-generator");
    write_page(
        &root,
        "src/pages/policies/[id]/page.rs",
        r#"#[ores_page(
renderer = "mash",
delivery = "ssr_only",
render = "static_only"
)]
pub async fn page() {}
"#,
    );
    let error = write_page_build_outputs(&root, &root.join(".ores-stack/first-pass"))
        .expect_err("dynamic static_only route without gen.rs must fail");
    let message = error.to_string().to_lowercase();
    assert!(message.contains("gen.rs") || message.contains("generator"), "{message}");
    cleanup(root);
}

fn rejects_ambiguous_dynamic_siblings() {
    let root = fresh_root("ambiguous-routes");
    for segment in ["[id]", "[slug]"] {
        write_page(
            &root,
            &format!("src/pages/evidence/{segment}/page.rs"),
            r#"#[ores_page(renderer = "mash", delivery = "ssr_only")]
pub async fn page() {}
"#,
        );
    }
    let error = write_page_build_outputs(&root, &root.join(".ores-stack/first-pass"))
        .expect_err("ambiguous dynamic siblings must fail");
    let message = error.to_string().to_lowercase();
    assert!(message.contains("ambiguous") || message.contains("conflict"), "{message}");
    cleanup(root);
}

fn rejects_unfinalized_browser_assets() {
    let root = fresh_root("unfinalized-browser-assets");
    write_page(
        &root,
        "src/pages/reports/[id]/page.rs",
        r#"#[ores_page(
renderer = "leptos",
delivery = "ssr_hydrate",
render = "dynamic",
client = "client.rs"
)]
pub async fn page() {}
"#,
    );
    let client = root.join("src/pages/reports/[id]/client.rs");
    fs::write(&client, b"pub fn hydrate() {}\n").expect("write browser client");

    let first_pass = root.join(".ores-stack/first-pass");
    let outputs = write_page_build_outputs(&root, &first_pass).expect("first pass must succeed");
    let manifest = read_page_build_manifest(&outputs.manifest_path).expect("manifest must parse");
    let wasm = manifest.routes[0].wasm.as_ref().expect("browser build plan");
    assert!(wasm.final_wasm_sha256.is_none());
    assert!(wasm.wasm_output_file.is_none());
    assert!(wasm.js_sha256.is_none());
    assert!(wasm.js_output_file.is_none());

    let materialized = root.join(".ores-stack/materialized");
    let error = materialize_finalized_page_build(
        &root,
        &materialized,
        &outputs.manifest_path,
        &first_pass.join("page-assets"),
    )
    .expect_err("unfinalized browser outputs must fail closed");
    let message = error.to_string().to_lowercase();
    assert!(
        message.contains("unfinalized") || message.contains("partially finalized"),
        "{message}"
    );
    assert!(
        !materialized.join("ores_pages.rs").exists(),
        "server glue must not be materialized from an unfinalized browser manifest"
    );
    cleanup(root);
}

fn write_page(root: &std::path::Path, relative: &str, source: &str) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().expect("page parent")).expect("create page parent");
    fs::write(path, source).expect("write page fixture");
}

fn fresh_root(suffix: &str) -> PathBuf {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock after epoch")
        .as_nanos();
    let root = env::temp_dir().join(format!(
        "canonical-ores-stack-fail-closed-{}-{now}-{suffix}",
        process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).expect("clear stale fixture");
    }
    root
}

fn cleanup(root: PathBuf) {
    fs::remove_dir_all(root).expect("cleanup fixture");
}
