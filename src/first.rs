// 单向链表实现栈数据结构
// List 的尾部不会再分配多余的 junk 值，通过!
// List 枚举的形式可以享受 null 指针优化，完美！
// 所有的元素都拥有统一的内存分配.
// Link
#[derive(Debug, Clone)]
enum Link {
    Empty,
    // List 不能确定大小需要用 Box 包裹,对于rust编译器来说所有栈上的类型都必须在编译期有固定的长度
    More(Box<Node>),
}

#[derive(Debug, Clone)]
struct Node {
    elem: i32,
    next: Link,
}

#[derive(Debug, Clone)]
pub struct List {
    head: Link,
}

impl List {
    fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, elem: i32) {
        let new_node: Box<Node> = Box::new(Node {
            elem: elem,
            // 函数允许我们从一个借用中偷出一个值的同时再放入一个新值。
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }
}
