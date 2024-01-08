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
            elem,
            // 函数允许我们从一个借用中偷出一个值的同时再放入一个新值。
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }
    // 出栈
    pub fn pop(&mut self) -> Option<i32> {
        // let result;
        // match std::mem::replace(&mut self.head, Link::Empty) {
        //     Link::Empty => {
        //         result = None;
        //     }
        //     Link::More(node) => {
        //         // 取出值
        //         result = Some(node.elem);
        //         // 把当前指向后面的节点赋值给前面
        //         self.head = node.next;
        //     }
        // };
        // result
        // 简化版本
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

// 模拟编译器实现释放
// impl Drop for List {
//     fn drop(&mut self) {
//         // NOTE: 在 Rust 代码中，我们不能显式的调用 `drop` 方法，只能调用 std::mem::drop 函数
//         // 这里只是在模拟编译器!
//         self.head.drop(); // 尾递归 - good!
//     }
// }
// impl Drop for Link {
//     fn drop(&mut self) {
//         match *self {
//             Link::Empty => {} // Done!
//             Link::More(ref mut boxed_node) => {
//                 boxed_node.drop(); // 尾递归 - good!
//             }
//         }
//     }
// }
// impl Drop for Box<Node> {
//     fn drop(&mut self) {
//         self.ptr.drop(); // 糟糕，这里不是尾递归!
//         deallocate(self.ptr); // 不是尾递归的原因是在 `drop` 后，还有额外的操作
//     }
// }
// impl Drop for Node {
//     fn drop(&mut self) {
//         self.next.drop();
//     }
// }

// 上述代码由于不是尾递归有可能太多撑破栈需要手动实现Drop
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
        // 递归释放
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node 在这里超出作用域并被 drop,
            // 由于它的 `next` 字段拥有的 `Node` 被设置为 Link::Empty,
            // 因此这里并不会有无边界的递归发生
        }
    }
}

// 测试
#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn test() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
