use ego_tree::{
    iter::{Children, NextSiblings},
    NodeRef,
};
use scraper::{ElementRef, Node};

pub trait ElementRefExt {
    fn children_elems(&self) -> ElemRefIter<Children<'_, Node>>;
    fn first_child_elem(&self) -> Option<ElementRef>;
    fn next_sibling_elems(&self) -> ElemRefIter<NextSiblings<'_, Node>>;
    fn text_as_string(&self) -> String;
    fn attr(&self, attr: &str) -> Option<&str>;
}

impl<'a> ElementRefExt for ElementRef<'a> {
    fn children_elems(&self) -> ElemRefIter<Children<'_, Node>> {
        ElemRefIter::from(self.children())
    }

    fn next_sibling_elems(&self) -> ElemRefIter<NextSiblings<'_, Node>> {
        ElemRefIter::from(self.next_siblings())
    }

    fn text_as_string(&self) -> String {
        let mut result = String::new();
        self.text().for_each(|t| {
            result.push_str(t.trim());
            result.push(' ');
        });
        result
    }

    fn attr(&self, attr: &str) -> Option<&str> {
        self.value().attr(attr)
    }

    fn first_child_elem(&self) -> Option<ElementRef> {
        self.children_elems().next()
    }
}

pub struct ElemRefIter<'a, I>(I)
where
    I: Iterator<Item = NodeRef<'a, Node>>;

impl<'a, I> ElemRefIter<'a, I>
where
    I: Iterator<Item = NodeRef<'a, Node>>,
{
    pub fn from(iter: I) -> Self {
        Self(iter)
    }
}

impl<'a, I> Iterator for ElemRefIter<'a, I>
where
    I: Iterator<Item = NodeRef<'a, Node>>,
{
    type Item = ElementRef<'a>;

    fn next(&mut self) -> Option<ElementRef<'a>> {
        loop {
            match self.0.next() {
                Some(n) => match ElementRef::wrap(n) {
                    Some(v) => return Some(v),
                    None => continue,
                },
                None => return None,
            }
        }
    }
}
