use std::cell::RefCell;
use std::rc::Rc;

// Rc<RefCell<T>> は、複数の所有権を持つことができ、内部の値を変更することができる。

// RefCell
// 内部可変性を提供するための型。実行時に借用規則をチェックする。
// borrow_mut ... RefCell のメソッド。内部の値の可変参照を取得
// into_inner ... RefCell のメソッド。RefCell の内部の値の所有権を取得

// Rc
// 参照カウントを使って所有権を共有するための型。
// 複数の所有権を持つことが可能
// clone ... Rc のメソッド。Rc の参照カウントを増やし新しいRcインスタンスを生成
// try_unwrap ... Rc のメソッド。Rc の参照カウントが1の場合、内部の値を取り出す（他に参照がある場合は、失敗する）所有権を取得

// Option
// take ... Option のメソッド。Some の値を取り出し、None を代入
// map ... Option のメソッド。Some の値にクロージャを適用し、Some の値を返す

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    value: T,
    prev: Link<T>,
    next: Link<T>,
}

pub struct DoubleLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> Default for DoubleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: self.head.clone(),
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            prev: self.tail.clone(),
            next: None,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    // head                                         tail
    //  ↓                                             ↓
    // Node1 (prev, next) -> Node2 (prev, next) -> Node3 (prev, next)

    // ↓↓↓

    // head                   tail
    //  ↓                      ↓
    // Node2 (prev, next) -> Node3 (prev, next)

    // ↓↓↓

    // head  tail
    //  ↓     ↓
    // Node3 (prev, next)

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
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().value
        })
    }

    // head                                         tail
    //  ↓                                             ↓
    // Node1 (prev, next) -> Node2 (prev, next) -> Node3 (prev, next)

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
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().value
        })
    }
}

fn main() {
    let mut list = DoubleLinkedList::<i32>::default();

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.push_front(4);
    list.push_front(5);

    assert_eq!(list.pop_front(), Some(5));
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_front(), Some(4));
    assert_eq!(list.pop_back(), Some(2));
}
