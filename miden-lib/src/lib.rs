#![cfg_attr(not(feature = "std"), no_std)]

use assembly::{Deserializable, Library, LibraryNamespace, MaslLibrary, Version};

pub mod memory;

// STANDARD LIBRARY
// ================================================================================================

pub struct MidenLib {
    contents: MaslLibrary,
}

impl Default for MidenLib {
    fn default() -> Self {
        let bytes = include_bytes!("../assets/miden.masl");
        let contents = MaslLibrary::read_from_bytes(bytes).expect("failed to read std masl!");
        Self { contents }
    }
}

impl Library for MidenLib {
    type ModuleIterator<'a> = <MaslLibrary as Library>::ModuleIterator<'a>;

    fn root_ns(&self) -> &LibraryNamespace {
        self.contents.root_ns()
    }

    fn version(&self) -> &Version {
        self.contents.version()
    }

    fn modules(&self) -> Self::ModuleIterator<'_> {
        self.contents.modules()
    }
}

#[test]
fn test_compile() {
    let path = "miden::sat::utils::get_consumed_note_data_ptr";
    let miden = MidenLib::default();
    let exists = miden.modules().any(|module| {
        module
            .ast
            .local_procs
            .iter()
            .any(|proc| module.path.concatenate(&proc.name).as_str() == path)
    });

    assert!(exists);
}
