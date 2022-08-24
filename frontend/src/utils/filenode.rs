use serde::{Deserialize, Serialize};
use yew::{Html, html};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileNode {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub children: Vec<FileNode>,
}

impl FileNode {
    pub fn new(id: u64, name: String) -> FileNode {
        FileNode {
            id,
            name,
            children: vec![],
        }
    }
    pub fn add_child(&mut self, child: FileNode) {
        self.children.push(child);
    }
    pub fn navigate<'a>(&'a self) -> FileNavigableNode {
        FileNavigableNode {
            node: self,
            parent: None,
        }
    }
    pub fn to_html(&self) -> Html{
        if self.children.len() > 0{
            return html!{
                <>
                <li class = "caret">{&self.name}</li>
                <ul class = "nested">{
                    (&self.children).into_iter().map(|file| file.to_html()).collect::<Html>()
                } </ul>
                </>
            }
        } else{
            return html!{
                <li>{&self.name}</li>
            }
        }
    }
}

/// For easier navigation
#[derive(Debug)]
pub struct FileNavigableNode<'a> {
    pub node: &'a FileNode,
    pub parent: Option<&'a FileNavigableNode<'a>>,
}

impl<'a> FileNavigableNode<'a> {
    pub fn child(&self, index: usize) -> FileNavigableNode {
        FileNavigableNode {
            node: &self.node.children[index],
            parent: Some(self),
        }
    }
}

