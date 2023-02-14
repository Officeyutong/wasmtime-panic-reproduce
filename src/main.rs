fn main() {
    let mut config = wasmtime::Config::new();
    config.wasm_component_model(true);
    let engine = wasmtime::Engine::new(&config).unwrap();
    wasmtime::component::Component::from_file(&engine, "./rust-bootstrap-component.wasm").unwrap();
}
