use std::mem;

pub struct List {
    head: Link,
}

/*  Link is just `Option<Box<Node>>`, so we can create a type alias for it.
    After this change, we also have to change all Link::Empty to None. */
type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None}
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,

            /*  This is a complex part of the code.
                `next` shouldn't take the ownership of `self.head, because list will be an
                uncompleted instance of List. So, with `mem::replace()`, we make a "temporary"
                borrow, then next can use self.head()*/
            next: mem::replace(&mut self.head, None)
        });

        self.head = Some(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        /*  First, we want to return an Option because we can make a pop in a empty List.
            Then, the option type let us return a None or a i32 value. */

        /*  We have to borrow `self.head`. */
        match mem::replace(&mut self.head, None) {

            None => None,
            Some(node)=> {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
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