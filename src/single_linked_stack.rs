use std::mem;

pub struct List<T> {
    head: Link<T>,
}

/*  Link is just `Option<Box<Node>>`, so we can create a type alias for it.
After this change, we also have to change all Link::Empty to None. */
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,

            /* This complex borrow operation can be done with Option.take. */
            next: self.head.take(),
        });

        self.head = Some(new_node)
    }

    pub fn pop(&mut self) -> Option<T> {
        /* We can replace this other complex operation with a closure,
        which is a lambda function that can access to local variables outside the scope of the closure.
        This will map any `Some(x)` to `Some(y)`, and leave None changeless. */

        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // With `drop`, we tells to the compiler how to deallocate unused values from `List`.

        let mut cur_link = mem::replace(&mut self.head, None);

        // `while let` == "do this thing until this pattern doesn't match"
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Test empty list
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
    }
}
