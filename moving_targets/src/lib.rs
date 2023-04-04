fn main() {
    let mut field = Field::new();

    println!("{:?}", field.pop());
    field.push(Target { size: 12, xp: 2 });
    println!("{:?}", *field.peek().unwrap());
    field.push(Target { size: 24, xp: 4 });
    println!("{:?}", field.pop());
    let last_target = field.peek_mut().unwrap();
    *last_target = Target { size: 2, xp: 0 };
    println!("{:?}", field.pop());
}

// level 5: moving_targets (karyun.cheung)

pub struct Field {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: Target,
    next: Link,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Target {
    pub size: u32,
    pub xp: u32,
}
impl Field {
    pub fn new() -> Self {
		Self {
			head: None,
		}
	}
    pub fn push(&mut self, target: Target) {
		let node = Node {
			elem: target,
			next: self.head.take(),
		};
		self.head = Some(Box::new(node));
	}
    pub fn pop(&mut self) -> Option<Target> {
		match self.head.take() {
			Some(link) => {
				self.head = link.next;
				Some(link.elem)
			}
			None => None,
		}
	}
    pub fn peek(&self) -> Option<&Target> {
		match &self.head {
			Some(link) => {
				Some(&link.elem)
			}
			None => None,
		}
	}
	pub fn peek_mut(&mut self) -> Option<&mut Target> {
		let mut node = self.head.as_mut()?;
		while let Some(next_node) = node.next.as_mut() {
			node = next_node;
		}
		Some(&mut node.elem)
	}
}
