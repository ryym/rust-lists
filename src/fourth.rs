use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

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

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail);
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

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    // 可能なら`RefCell`という実装の詳細を隠し、`Option<&T>`を返したいがそれはできない。
    // `RefCell`が行うのは、「`&T`と`&mut T`は同時に存在できない」というルールと同じ制約を
    // `Ref`と`RefMut`に実行時に課す事であり、
    // そのためには`&T`ではなく`Ref`の生存期間が重要になる。
    // つまり`Ref`をここで作って`&T`だけを返す事ができてしまうと、
    // `Ref`は関数の終了とともに消えてしまうため、
    // 「`Ref`と`RefMut`は同時に存在できない」というルールを破らないまま
    // `&T`が生き残ってしまう。
    // そのため`RefCell`の値の参照が必要な場合には、
    // あくまで`&T`ではなく`Ref`を使う必要がある。
    // だから`Ref`から得られる参照の lifetime は`RefCell`ではなく`Ref`に紐付いている。
    // pub fn peek_front(&self) -> Option<&T> {
    //     self.head.as_ref().map(|head| &head.borrow().elem)
    // }
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|head| {
            Ref::map(head.borrow(), |node| &node.elem)
        })
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_mut().map(|head| {
            RefMut::map(head.borrow_mut(), |node| &mut node.elem)
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|tail| {
            Ref::map(tail.borrow(), |node| &node.elem)
        })
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_mut().map(|tail| {
            RefMut::map(tail.borrow_mut(), |node| &mut node.elem)
        })
    }
}

impl<T> Drop for List<T> {
    // doubly-linked list なので、`Rc`に包まれた各ノードは互いの参照を持ち合う。
    // このような相互 (循環) 参照がある場合、`Rc`は正しくメモリを解放しない
    // (`List`を削除しても両端の要素の参照数が1つ減るだけ)。
    // そのため手動でノードを辿って削除していく必要がある。
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        let mut list = List::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        list.push_back(4);
        list.push_back(5);
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek_front() {
        let mut list = List::new();

        list.push_front(1);
        assert_eq!(&*list.peek_front().unwrap(), &1);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 1);

        list.push_front(2);
        assert_eq!(&*list.peek_front().unwrap(), &2);
    }

    #[test]
    fn peek_back() {
        let mut list = List::new();

        list.push_front(1);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);

        list.push_front(2);
        assert_eq!(&*list.peek_back().unwrap(), &1);

        list.push_back(3);
        assert_eq!(&*list.peek_back().unwrap(), &3);
    }
}
