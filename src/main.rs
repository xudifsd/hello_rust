use std::ops::Add;

use std::rc::Rc;

extern crate tree;

use tree::Tree;

////////

struct Node<'a, T> where T: 'a {
    val: &'a T,
    left: Option<Box<Node<'a, T>>>,
    right: Option<Box<Node<'a, T>>>,
}

impl<'a, T> Node<'a, T> where T: PartialOrd {
    pub fn new(val: &'a T) -> Node<'a, T> {
        Node {val: val, left: None, right: None}
    }
    pub fn insert(&mut self, val: &'a T) {
        if self.val == val {
            return
        }
        let target_node = if val < self.val { &mut self.left } else { &mut self.right };
        if let &mut Some(ref mut child) = target_node {
            child.insert(val);
        } else {
            let new = Node { val: val, left: None, right: None};
            let boxed = Some(Box::new(new));
            *target_node = boxed;
        }
    }
}

#[test]
fn test_bst() {
    let one = 1;
    let two = 2;
    let p1 = &one;
    let p2 = &two;

    let mut r = Node::new(p1);
    r.insert(p2);
}

////////////

trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct Square {
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn get_area<T>(t: T) -> f64 where T: HasArea {
    t.area()
}

#[test]
fn test_get_area() {
    let c = Circle {radius: 3.0};
    let s = Square {side: 3.0};
    assert!(get_area(c) == std::f64::consts::PI * 9.0);
    assert!(get_area(s) == 9.0);
}

///////////////

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Add<T> for Point<T> where T: Add<T,Output=T> + Copy {
    type Output = Point<T>;

    fn add(self, rhs: T) -> Point<T> {
        Point { x: rhs + self.x, y: rhs + self.y }
    }
}

#[test]
fn test_add_point() {
    let p1 = Point {x: 1.0, y: 2.0};
    let Point {x: x1, y: y1} = p1 + 3.1;
    assert!(x1 == 4.1 && y1 == 5.1);
}

////////////

struct ListNode<'a, T> where T: PartialOrd + 'a {
    val: &'a T,
    next: Option<Box<ListNode<'a, T>>>,
}

struct List<'a, T> where T: PartialOrd + 'a {
    head: Option<Box<ListNode<'a, T>>>,
}

impl<'a, T> List<'a, T> where T: PartialOrd + 'a {
    fn new() -> List<'a, T> {
        List{ head: None }
    }
    fn insert(&mut self, val: &'a T) {
        // http://stackoverflow.com/a/28268208/845762
        let tail = self.head.take();
        self.head = Some(Box::new(ListNode{ val: val, next: tail}));
    }
    fn pop(&mut self) -> Option<&'a T> {
        let result = self.head.take();
        if let Some(x) = result {
            let ListNode {val, next} = *x;
            self.head = next;
            Some(val)
        } else {
            None
        }
    }
    // it seems we can not implement reverse using &mut self, because
    // we have to own it to implement
    fn reverse(mut self) -> List<'a, T> {
        if let Some(mut p) = self.head {
            let mut tail = None;
            loop {
                let p_next = p.next.take();
                p.next = tail;
                tail = Some(p);
                match p_next {
                    None => break,
                    _    => p = p_next.unwrap(),
                };
            };
            self.head = tail;
        }
        self
    }
}

#[test]
fn test_list_insert_pop() {
    let one = 1;
    let two = 2;
    let p1 = &one;
    let p2 = &two;
    let mut head = List::new();

    head.insert(p1);
    head.insert(p2);

    let p3 = head.pop();
    let p4 = head.pop();

    match (p3, p4) {
        (Some(p3), Some(p4)) => assert!(p3 == p2 && p4 == p1),
        _ => assert!(false),
    };
}

#[test]
fn test_reverse() {
    let one = 1;
    let two = 2;
    let p1 = &one;
    let p2 = &two;
    let mut head = List::new();

    head.insert(p1);
    head.insert(p2);
    head = head.reverse();

    let p3 = head.pop();
    let p4 = head.pop();

    match (p3, p4) {
        (Some(p3), Some(p4)) => assert!(p3 == p1 && p4 == p2),
        _ => assert!(false),
    };
}

///////////

struct StackFrame<T> {
    val: T,
    next: Option<Box<StackFrame<T>>>,
}

struct Stack<T> {
    top: Option<Box<StackFrame<T>>>,
}

// I have to bound T under Copy, because otherwise `let StackFrame{val, next} = *x;`
// will get error "use of moved value: `x`", but I do not understand why NotCopy below
// works
impl<T> Stack<T> where T: Copy {
    fn new() -> Stack<T> {
        Stack {top: None}
    }
    fn push(&mut self, val: T) {
        let top = self.top.take();
        self.top = Some(Box::new(StackFrame{ val: val, next: top}));
    }
    fn pop(&mut self) -> Option<T> {
        let top = self.top.take();
        if let Some(x) = top {
            let StackFrame{val, next} = *x;
            self.top = next;
            Some(val)
        } else {
            None
        }
    }
}

macro_rules! assert_some_and_eq {
    ($exp: expr, $val: expr) => {
        if let Some(x) = $exp {
            assert_eq!(x, $val);
        } else {
            assert!(false);
        }
    }
}

#[test]
fn test_stack() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    assert!(s.pop().unwrap() == 3);
    assert!(s.pop().unwrap() == 2);
    assert_some_and_eq!(s.pop(), 1);
    assert!(s.pop().is_none());
}

//////

struct NotCopy {
    dummy: i32,
}

struct Owner {
    a: NotCopy,
    b: NotCopy,
}

#[test]
fn test_struct_ownership() {
    let o = Owner{ a: NotCopy{ dummy: 1}, b: NotCopy{ dummy: 2}};
    let Owner{a, b} = o;
    assert!(a.dummy == 1);
    assert!(b.dummy == 2);
}

struct Foo {
    val: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("dropping");
    }
}

fn test_drop() {
    {
        let rc1 = Rc::new(Box::new(Foo{val: 42}));
        println!("aaa");
        {
            let rc2 = rc1.clone();
            println!("bbb");
        }
        println!("ccc");
    }
    println!("out scope");
}

fn main() {
    test_drop();
    println!("-----------");

    let mut t = Tree::new();
    t.insert(1);
    t.insert(100);
    t.insert(50);
    assert_eq!(t.into_vec(), vec![&1, &50, &100]);
}
