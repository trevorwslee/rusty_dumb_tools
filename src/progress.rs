#![deny(warnings)]
#![allow(unused)]

use std::{thread, time::Duration};

#[test]
fn debug_progress() {
    if true {
        let items = vec![
            String::from("apple"),
            String::from("banana"),
            String::from("cherry"),
            String::from("date"),
        ];
        let mut iter = DumbProgressIndicator::new(Box::new(items.iter()));
        while let Some(item) = iter.next() {
            println!("Processing item: {}", item);
        }
    }
    if true {
        let items = vec![
            String::from("apple"),
            String::from("banana"),
            String::from("cherry"),
            String::from("date"),
        ];
        let mut iter = DumbProgressIndicator::new(Box::new(items.into_iter()));
        while let Some(item) = iter.next() {
            println!("Processing item: {}", item);
        }
    }

    if true {
        let items = vec!["apple", "banana", "cherry", "date"];
        let mut iter = DPIVector::new(&items);
        while let Some(item) = iter.next() {
            println!("Processing item: {}", item);
        }
    }

    if true {
        let items = vec![
            String::from("apple"),
            String::from("banana"),
            String::from("cherry"),
            String::from("date"),
        ];
        let mut owned_item_iter = items.into_iter();
        let mut iter = DPIIntoIter::new(&mut owned_item_iter);
        while let Some(item) = iter.next() {
            println!("Processing item: {}", item);
        }
    }

    if true {
        let items = vec!["apple", "banana", "cherry", "date"];
        let mut item_iter = items.iter();
        let mut iter = DPISliceIter::new(&mut item_iter);
        while let Some(item) = iter.next() {
            println!("Processing item: {}", item);
        }
    }
}

// fn test<'a, T>(i: Box<dyn Iterator<Item = T> + 'a>) {
// }

struct DumbProgressIndicator<'a, T> {
    boxed_iterator: Box<dyn Iterator<Item = T> + 'a>,
}

impl<'a, T> DumbProgressIndicator<'a, T> {
    pub fn new(boxed_iterator: Box<dyn Iterator<Item = T> + 'a>) -> DumbProgressIndicator<T> {
        DumbProgressIndicator { boxed_iterator }
    }
}

impl<'a, T> Iterator for DumbProgressIndicator<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.boxed_iterator.as_mut().next()
    }
}

struct DPIVector<'a, T> {
    items: &'a Vec<T>,
    current: usize,
}

impl<'a, T> DPIVector<'a, T> {
    pub fn new(items: &'a Vec<T>) -> DPIVector<'a, T> {
        DPIVector { items, current: 0 }
    }
}

impl<'a, T> Iterator for DPIVector<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.items.len() {
            let item = &self.items[self.current];
            self.current += 1;
            Some(item)
        } else {
            None
        }
    }
}

struct DPIIntoIter<'a, T> {
    iter: &'a mut std::vec::IntoIter<T>,
}

impl<'a, T> DPIIntoIter<'a, T> {
    pub fn new(iter: &'a mut std::vec::IntoIter<T>) -> DPIIntoIter<'a, T> {
        DPIIntoIter { iter }
    }
}

impl<'a, T> Iterator for DPIIntoIter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

struct DPISliceIter<'a, T> {
    iter: &'a mut std::slice::Iter<'a, T>,
}

impl<'a, T> DPISliceIter<'a, T> {
    pub fn new(iter: &'a mut std::slice::Iter<'a, T>) -> DPISliceIter<'a, T> {
        DPISliceIter { iter }
    }
}

impl<'a, T> Iterator for DPISliceIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
