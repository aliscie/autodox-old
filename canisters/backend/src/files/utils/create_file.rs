// use shared::id::Id;
// use shared::schema::{FileDirectory, FileNode};
// use crate::users::types::UserFiles;
//
// pub fn _create_file(user_files: &mut UserFiles, username: &String, directory_id: Id, id: Id, name: String, children: Option<Vec<Id>>) -> CreateResult{
//     let filenode = FileNode{
//         id,
//         name,
//         element_tree: None,
//     };
//     let mut default = Vec::new();
//     let directories = user_files.get_mut(username).unwrap_or(&mut default);
//     for directory in directories.iter_mut(){
//         if directory.id == directory_id{
//             directory.files.vertices.insert(filenode.id.clone(), filenode.clone());
//             directory.files.adjacency.insert(filenode.id.clone(), children.unwrap_or(Vec::new()));
//             return CreateResult::Ok
//         }
//     }
//     CreateResult::DirectoryDoesNotExist
// }
//
// pub fn _create_directory(user_files: &mut UserFiles, username: &String, directory_id: Id, name: String) -> CreateResult{
//     let fd = FileDirectory::new(directory_id, name);
//     match user_files.get_mut(username){
//         Some(files) => {
//             if files.iter().any(|file| file.id == directory_id){
//                 return CreateResult::AlreadyExist
//             }
//             files.push(fd);
//             return CreateResult::Ok
//         },
//         None => ()
//     }
//     let l_fd = vec![fd];
//     user_files.insert(username.clone(), l_fd);
//     return CreateResult::Ok
// }
