use shared::schema::FileDirectory;
use shared::*;
use yewdux::prelude::Dispatch;
use std::collections::HashMap;
use shared::schema::ElementTree;
use crate::backend;
use shared::id::Id;
use yewdux::functional::use_store;
use yewdux::store::Store;

// #[derive(Debug, PartialEq, Eq, Clone, Store)]
// pub struct ElementTreeStore {
//     id: Id,
//     element_tree: ElementTree,
// }

#[derive(Debug, PartialEq, Eq, Clone, Store, Default)]
pub struct ElementTreeStore {
    pub map: HashMap<Id, ElementTree>,
}

pub async fn init_files() -> Result<(), String> {
    let dispatch_file_directory = Dispatch::<FileDirectory>::new();
    let dispatch_element_tree = Dispatch::<ElementTreeStore>::new();
    // let (element_tree, dispatch_element_tree) = use_store::<ElementTreeStore>();
    //  how to save hashmap in yewdux


    let mut element_trees = crate::backend::get_element_trees().await
        .map_err(|e| e);

    let mut directories = crate::backend::get_directories()
        .await
        .map_err(|e| e)
        .unwrap_or_default();
    let dispatch_file_directory = Dispatch::<FileDirectory>::new();

    match directories {
        Some(directories) => {
            dispatch_file_directory.set(directories);
        }
        None => {
            let file_directory = FileDirectory::default();
            let x = crate::backend::create_directory(&file_directory).await;
            dispatch_file_directory.set(file_directory);
        }
    }

    log!(&element_trees);
    // Todo save element trees in the yewdux store
    match element_trees {
        Ok(element_trees) => {
            dispatch_element_tree.set(ElementTreeStore { map: element_trees });
        }
        _ => {
            // let file_directory = ElementTree::default();
            // let x = crate::backend::create_element_tree(&file_directory).await;
            // dispatch_element_tree.set(HashMap::new());
        }
    }

    Ok(())
}
