use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CompilerSettings {
    pub compiler: String,
    pub compiler_args: String,
    pub debug_args: String,
    pub executable_name: String,
}