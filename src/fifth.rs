// - singly-linked queue を実装したい。
// - Listが`head`しか持ってないと、 push や pop に O(n) かかる
//   (毎回最後のノードまで辿らないといけない)
// - `List`が最後のノードも持ってればいい!
// - 所有権の問題があるため、`tail: Link<T>`とは書けない。
//   (最後のノードはその手前のノードの`next`が所有する)
// - `tail: Option<&mut Node<T>>`と書いて参照を保持しよう!
// - その参照の lifetimeは? ...Listと同じ?
//   (`List<'a, T: 'a> { tail: Option<&'a mut Node<T>>, head: Link<T> }`)
// - しかしこれは間違っている。`List`が生きている限り、
//   本当は既に無効な参照を`tail`が持ち続けてしまう可能性がある。
// - このlifetimeを適切に設定する方法がない..?

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }
}
