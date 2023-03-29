#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }
    pub fn push(&mut self, value: T) {
        let new_node = Node {
            value,
            next: self.head.take().map(|node|Box::new(node)),
        };
        self.head = Some(new_node);
    }
    pub fn pop(&mut self) {
        self.head.take().map(|node| {
            self.head = node.next.map(|boxed_node| *boxed_node);
        });
    }
    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut node = self.head.as_ref();
        while let Some(curr_node) = node {
            len += 1;
            node = curr_node.next.as_ref().map(|boxed_node| &**boxed_node);
        }
        len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut new_list_str = List::new();
        new_list_str.push("String Test 1");
        println!("The size of the list is {}", new_list_str.len());
    
        new_list_str.push("String Test 2");
        println!("The size of the list is {}", new_list_str.len());
    
        new_list_str.push("String Test 3");
        println!("The size of the list is {}", new_list_str.len());
    
        new_list_str.pop();
        println!("The size of the list is {}", new_list_str.len());
    }
}
