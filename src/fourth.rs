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

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            // 正しく実装できていれば`old_head`の参照は他にないはずなので
            // `ok`と`unwrap`を使ってエラーケースを無視する。
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
}
