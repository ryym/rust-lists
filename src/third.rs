use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

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
            head: Some(Rc::new(Node { elem, next })),
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
}
