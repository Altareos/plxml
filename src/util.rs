use roxmltree::Node;

pub fn tag_name(node: &Node) -> String {
    node.tag_name().name().to_lowercase()
}

pub fn find_node<'a>(node: &'a Node<'a, 'a>, tag: &str) -> Option<Node<'a, 'a>> {
    node.children().find(|n| tag_name(n) == tag)
}
