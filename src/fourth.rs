use std::rc::Rc;
use std::cell::RefCell;

// `RefCell`からは`Ref`と`RefMut`を取得でき、これらは`&`と`&mut`と同じ
// ルールを持っている (`Ref`はいくつでも作れるが、`RefMut`は同時に1つだけ)。
// ただし、このルールをコンパイル時にではなくランタイム時にチェックし、
// ルールが破られた場合は panic を起こす。

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }
}
