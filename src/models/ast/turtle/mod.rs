use oxrdf;

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    pub triples: Vec<Triple>,
}

#[derive(Debug)]
pub struct Triple {
    pub subject: Node,
    pub predicate: Node,
    pub object: Node,
}

#[derive(Debug)]
pub enum Node {
    IRI(String),
    Literal(String),
    BlankNode(String),
}

impl From<oxrdf::Term> for Node {
    fn from(term: oxrdf::Term) -> Self {
        match term {
            oxrdf::Term::NamedNode(node) => Node::IRI(node.as_str().to_string()),
            oxrdf::Term::Literal(lit) => Node::Literal(lit.value().to_string()),
            oxrdf::Term::BlankNode(bnode) => Node::BlankNode(bnode.as_str().to_string()),
            _ => unimplemented!(),
        }
    }
}
