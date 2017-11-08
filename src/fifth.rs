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

use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        // *mut な raw pointer は nullable なので、Optionを使う意味がない。
        // null を None 代わりに使う。ただし Java などの null とは違い、
        // null も各種メソッドを持った primitve type (raw pointer) となる。
        List { head: None, tail: ptr::null_mut() }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node { elem, next: None, });

        // 通常の値を raw pointer にするには、 raw pointer型として deref する。
        let raw_tail: *mut _ = &mut *new_tail;

        if self.tail.is_null() {
            self.head = Some(new_tail);
        } else {
            // raw pointer 内の値にアクセスする場合は、明示的に deref する必要がある。
            // また、deref された値へのアクセスは常に unsafe としてマークされる。
            // 逆に`is_null`のような raw pointer型が持つメソッドの呼び出しや、
            // pointer 自体への代入は safe と言えるので unsafe ブロックは不要。
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
            // struct のフィールドはデフォルトだとモジュール外からは private なので、
            // このライブラリ内の操作さえ安全に書ければ、外から見たインターフェースは
            // 通常の Rust と同じ安全なものになるはず。
        }

        self.tail = raw_tail;
    }
}
