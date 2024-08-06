use std::mem;

/*  This way to declare a Linked List is really bad. A good linked list should use `only` one
    struct. But this is the easy way to do it. */

pub struct List {
    /* A new Linked List is created with a head, which is a Link Type. */
    head: Link,
}

enum Link {
    /* This link type is useful to avoid structs infinite recursion (or kinda of). Idk, I
        didn't understood this part of the guide... */
    Empty,
    More(Box<Node>),
}

struct Node {
    /* I hope this part of the code doesn't need an explanation. */
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,

            /*  This is a complex part of the code.
                `next` shouldn't take the ownership of `self.head, because list will be an
                uncompleted instance of List. So, with `mem::replace()`, we make a "temporary"
                borrow, then next can use self.head()*/
            next: mem::replace(&mut self.head, Link::Empty)
        });

        self.head = Link::More(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        /*  First, we want to return an Option because we can make a pop in a empty List.
            Then, the option type let us return a None or a i32 value. */

        /*  We have to borrow `self.head`. */
        match mem::replace(&mut self.head, Link::Empty) {

            Link::Empty => None,
            Link::More(node)=> {
                self.head = node.next;
                Some(node.elem)
            }
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