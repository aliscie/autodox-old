use yew::prelude::*;
use yewdux::prelude::*;

use shared::schema::FileDirectory;

#[derive(PartialEq, Properties)]
pub struct TreeListProps {
    pub title: Option<String>,
    pub children: Option<Children>,
}

#[function_component(TreeList)]
pub fn tree_list(props: &TreeListProps) -> Html {
    let (tree, _) = use_store::<FileDirectory>();
    //d.reduce_mut(|r| r.data.insert(235, "file two!".into()));
    //d.reduce_mut(|r| r.data.insert(224, "file three!".into()));
    //d.reduce_mut(|r| r.data.insert(225, "file four!".into()));

    //(props.files.clone()).into_iter().map(|file| file.to_html()).collect::<Html>()
    crate::utils::filetree::to_html(&tree, tree.files.root.unwrap())
}
