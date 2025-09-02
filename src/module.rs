use crate::python_utils::split_import;

pub struct PythonModule {
    name: String,
    long_name: Vec<String>
}

impl PythonModule {
    pub fn new(name: &str, long_name: &str) -> Self {
        PythonModule { name: String::from(name), long_name: split_import(long_name) }
        
    }
}