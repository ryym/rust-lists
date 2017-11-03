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
