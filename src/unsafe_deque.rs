// fifth.rs

// - singly-linked queue を実装したい。
// - Listが`head`しか持ってないと、 push に O(n) かかる
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

// わずかに unsafe な操作を導入する事で、
// RefCellを使う諸々の面倒さを避ける事はできている。

use std::ptr;

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

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

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;

            if self.head.is_none() {
                // もしこの null 化処理を忘れたら、次の push はおかしな場所にノードを
                // 挿入することになる。しかしコンパイル時にはそれがわからない。
                self.tail = ptr::null_mut();
            }

            node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|n| &**n);
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        // XXX: なぜ &mut の場合だけ`take`が必要なのかわからない。。
        // 直接`map`すると`cannot move out of borrowed content`になる。
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|n| &mut **n);
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1); list.push(2); list.push(3);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        list.push(4); list.push(5);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        list.push(6); list.push(7);
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
    }
}
