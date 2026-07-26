use std::{
    collections::HashMap,
    fmt::Debug,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use jloader::class_file::ClassLoc;

use crate::{class, errors::exceptions::Exception, vm::VM};

#[derive(PartialEq, Debug)]
enum LoadState {
    Initialized,
    Initializing,
    Loading,
    Loaded,
}

pub trait ClassLoader {
    fn load_class(
        &mut self,
        class: String,
        method_area: Arc<Mutex<Vec<ClassLoc>>>,
        heap: Arc<Mutex<Vec<u8>>>,
    ) -> Result<ClassLoc, Exception>;

    fn debug(&self) -> String;
}

pub struct BootstrapClassloader {
    // Path of core containing crap
    class_path: PathBuf,
    class_state: HashMap<String, LoadState>,
    loaded: HashMap<String, ClassLoc>,
}

impl BootstrapClassloader {
    pub fn new(class_path: PathBuf) -> BootstrapClassloader {
        BootstrapClassloader {
            class_path,
            class_state: HashMap::new(),
            loaded: HashMap::new(),
        }
    }
}

impl ClassLoader for BootstrapClassloader {
    fn load_class(
        &mut self,
        class: String,
        method_area: Arc<Mutex<Vec<ClassLoc>>>,
        heap: Arc<Mutex<Vec<u8>>>,
    ) -> Result<ClassLoc, Exception> {
        // Has this loader already loaded this class/interface? If so just return the existing class
        if let Some(state) = self.class_state.get(&class) {
            if *state == LoadState::Loaded {
                let Some(loc) = self.loaded.get(&class) else {
                    return Err(Exception::ClassNotFound);
                };
                return Ok(loc.clone());
            }
        } else {
            let mut heap = heap.lock().unwrap();
            let mut method_area = method_area.lock().unwrap();
            let loaded_class = class::load_class(
                &mut heap,
                &mut method_area,
                &self
                    .class_path
                    .join(class.clone())
                    .with_added_extension("class"),
            )
            .map_err(Exception::Other)?;
            let loc = method_area.last().unwrap();
            self.loaded.insert(class.clone(), loc.clone());
            self.class_state.insert(class, LoadState::Loaded);

            return Ok(loc.clone());
        }

        todo!()
    }

    fn debug(&self) -> String {
        format!("BootstrapClassloader {{\n\tclass_path: {:?}\n\tclass_state: {:?}\n\tloaded: {:?}\n}}", self.class_path, self.class_state, self.loaded)
    }
}
