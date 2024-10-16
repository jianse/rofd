use minidom::Node;

pub trait ToNode {
    fn to_node(&self) -> Option<Node>;
}
pub mod impls {
    use crate::ToNode;
    use minidom::Node;

    impl ToNode for String {
        fn to_node(&self) -> Option<Node> {
            Some(Node::Text(self.clone()))
        }
    }
    impl ToNode for u8 {
        fn to_node(&self) -> Option<Node> {
            todo!()
        }
    }
    impl<T> ToNode for Option<T>
    where
        T: ToNode,
    {
        fn to_node(&self) -> Option<Node> {
            match self {
                None => None,
                Some(e) => e.to_node(),
            }
        }
    }

    impl<T> ToNode for Vec<T>
    where
        T: ToNode,
    {
        fn to_node(&self) -> Option<Node> {
            todo!()
        }
    }

    impl ToNode for dyn ToString {
        fn to_node(&self) -> std::option::Option<minidom::Node> {
            self.to_string().to_node()
        }
    }
}
