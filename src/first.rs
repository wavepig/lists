// 单向链表实现栈数据结构
// List 的尾部不会再分配多余的 junk 值，通过!
// List 枚举的形式可以享受 null 指针优化，完美！
// 所有的元素都拥有统一的内存分配.
pub struct List {
    head: Link,
}

// Link
enum Link {
    Empty,
    // List 不能确定大小需要用 Box 包裹,对于rust编译器来说所有栈上的类型都必须在编译期有固定的长度
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: List,
}
