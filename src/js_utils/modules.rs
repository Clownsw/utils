use crate::js_utils::adapters::JsRealmAdapter;
use std::sync::Arc;

pub trait ScriptModuleLoader {
    fn normalize_path(&self, ref_path: &str, path: &str) -> Option<String>;
    fn load_module(&self, absolute_path: &str) -> String;
}

pub trait CompiledModuleLoader<R: JsRealmAdapter> {
    fn normalize_path(&self, q_ctx: &R, ref_path: &str, path: &str) -> Option<String>;
    fn load_module(&self, q_ctx: &R, absolute_path: &str) -> Arc<Vec<u8>>;
}

pub trait NativeModuleLoader<R: JsRealmAdapter> {
    fn has_module(&self, q_ctx: &R, module_name: &str) -> bool;
    fn get_module_export_names(&self, q_ctx: &R, module_name: &str) -> Vec<&str>;
    fn get_module_exports(
        &self,
        q_ctx: &R,
        module_name: &str,
    ) -> Vec<(&str, R::JsValueAdapterType)>;
}
