struct Attributes {
    style: string,
    class: string,
    id: string,
    href: string,
    src: string,
    contenteditable: bool,
}

pub struct Content {
    text: String,
    tag: String,
    id: String,
    attributes: HashMap<Attributes>,
    children: Vec<Content>,
}

struct Position {
    crate_position: Option<i32>,
}
