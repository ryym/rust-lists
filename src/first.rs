// http://cglab.ca/~abeinges/blah/too-many-lists/book/first-layout.html

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
}
