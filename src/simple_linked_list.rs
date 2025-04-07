// pub struct SimpleLinkedList<T> {
//     // Delete this field
//     // dummy is needed to avoid unused parameter error during compilation
//     dummy: ::std::marker::PhantomData<T>,
// }

// impl<T> SimpleLinkedList<T> {
//     pub fn new() -> Self {
//         todo!()
//     }

//     // You may be wondering why it's necessary to have is_empty()
//     // when it can easily be determined from len().
//     // It's good custom to have both because len() can be expensive for some types,
//     // whereas is_empty() is almost always cheap.
//     // (Also ask yourself whether len() is expensive for SimpleLinkedList)
//     pub fn is_empty(&self) -> bool {
//         todo!()
//     }

//     pub fn len(&self) -> usize {
//         todo!()
//     }

//     pub fn push(&mut self, _element: T) {
//         todo!()
//     }

//     pub fn pop(&mut self) -> Option<T> {
//         todo!()
//     }

//     pub fn peek(&self) -> Option<&T> {
//         todo!()
//     }

//     #[must_use]
//     pub fn rev(self) -> SimpleLinkedList<T> {
//         todo!()
//     }
// }

// impl<T> FromIterator<T> for SimpleLinkedList<T> {
//     fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
//         todo!()
//     }
// }

// // In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// // instead of implementing an explicit conversion to a vector. This is because, together,
// // FromIterator and IntoIterator enable conversion between arbitrary collections.
// //
// // The reason this exercise's API includes an explicit conversion to Vec<T> instead
// // of IntoIterator is that implementing that interface is fairly complicated, and
// // demands more of the student than we expect at this point in the track.
// //
// // Please note that the "front" of the linked list should correspond to the "back"
// // of the vector as far as the tests are concerned.

// impl<T> From<SimpleLinkedList<T>> for Vec<T> {
//     fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
//         todo!()
//     }
// }

pub struct SimpleLinkedList<T> {
    length: usize,
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            data: element,
            next: self.head.take(),
        });
        
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.length -= 1;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut result = SimpleLinkedList::new();
        let mut current = self;
        
        while let Some(data) = current.pop() {
            result.push(data);
        }
        
        result
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for item in iter {
            list.push(item);
        }
        list
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = Vec::with_capacity(linked_list.len());
        
        // Pop items from the list (which gives us the elements in reversed order)
        // and then reverse the vector at the end
        while let Some(item) = linked_list.pop() {
            vec.push(item);
        }
        
        // Since we want [1, 2, 3] instead of [3, 2, 1], we need to reverse
        vec.reverse();
        
        vec
    }
}