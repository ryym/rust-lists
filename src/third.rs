use std::sync::Arc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Arc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn append(&self, elem: T) -> List<T> {
        // Copy trait を実装している型の値を別の変数に代入した場合、
        // 元の変数からCopyされるため、元の変数にも引き続きアクセスできる。
        // このようにCopyは暗黙的でカスタマイズできないが、Cloneは明示的な
        // 値のコピーであり、ユーザが実装を定義できる。
        let next = self.head.clone();
        List {
            head: Some(Arc::new(Node { elem, next })),
        }
    }

    pub fn tail(&self) -> List<T> {
        // `and_then` is like a `flat_map`.
        let head = self.head.as_ref().and_then(|node| node.next.clone());
        List { head }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

// 状態変更をしない実装のため、`third::List`の`Iter`や`IterMut`は実装できない。

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            // `node`が最後の参照だった場合 (他に参照を保持している箇所がない場合) のみ
            // `try_unwrap`が成功し、destruction する。
            if let Ok(mut node) = Arc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));
        assert_eq!(list.tail().head(), Some(&2));
        assert_eq!(list.tail().tail().head(), Some(&1));
        assert_eq!(list.tail().tail().tail().head(), None);
        assert_eq!(list.tail().tail().tail().tail().head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().append(1).append(2);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(list.head(), Some(&2));
    }
}
