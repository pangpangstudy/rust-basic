// Box 是 Rust 中用于在堆上分配内存的智能指针。它允许将数据存储在堆上，而不是栈上。
// dyn State dyn 关键字表示这是一个 trait 对象 State 在这里是一个 trait（接口）的名称
// Box<dyn State> 的组合作用 它创建了一个指向堆上分配的、实现了 State trait 的对象的指针 这允许使用动态分发，即在运行时决定调用哪个具体实现的方法。
// Box 拥有它指向的数据，当 Box 被丢弃时，它指向的数据也会被释放
/// 本结构体表示一个已发布的博文
pub struct Post {
    content: String,
}

impl Post {
    /// 本方法用于创建一个草稿状态的博文
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    /// 本方法用于获取博文内容
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// 本结构体表示一个草稿状态的博文
pub struct DraftPost {
    content: String,
}

impl DraftPost {
    /// 本方法用于向草稿中添加文本
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    /// 本方法用于申请审批博文
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

/// 本结构体表示一个待审批状态的博文
pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    /// 本方法用于审批通过博文
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();
    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
