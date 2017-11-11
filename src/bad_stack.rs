// first.rs

// コンパイルは通るけどメモリ効率に無駄がある。
// pub enum List {
//     Empty,
//     Elem(i32, Box<List>),
// }

// これを public にすると Node も public にしないといけない。
// 実装の詳細は隠したい。
// pub enum List {
//     Empty,
//     More(Box<Node>),
// }
// struct Node {
//     elem: i32,
//     next: List,
// }

use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        // 借用中の self の値はムーブできない。
        // `mem::replace`を使って代わりの値をセットする事でムーブできちゃう。
        let next = mem::replace(&mut self.head, Link::Empty);
        let new_node = Node { elem: elem, next: next };
        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        // head の値を得る。これにより、値の所有権はこのローカル変数に移る。
        // 単に`match self.head {...}`としてしまうと、`self.head`の値を
        // match式が借用 (borrow) する形になり、match式内で`self.head`を書き換えられない。
        let head = mem::replace(&mut self.head, Link::Empty);
        match head {
            Link::Empty => None,
            Link::More(boxed_node) => {
                let node = *boxed_node;
                self.head = node.next;
                Some(node.elem)

                // これはできない。
                // next を使った時点で Box 全体の所有権がなくなる..?
                // self.head = boxed_node.next;
                // Some(node.elem)
            }
        }
    }
}

// デフォルトの destructor は再帰的にノードをたどるため、
// リストが大きいと stack overflow が起きうる
// (Box を挟むデータ構造の destruction は末尾再帰にならない)。
// よって自前で Drop を実装する。
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // `boxed_node`はここでスコープから消え、drop される。
            // `boxed_node.next`は既に`Link::Empty`になっているため、
            // 再帰的な drop も発生しない。
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
