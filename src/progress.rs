#![deny(warnings)]
#![allow(unused)]

use std::{cell::RefCell, io::{self, Write}, mem::MaybeUninit, sync::Once, thread, time::Duration};


#[macro_export]
macro_rules! dpiter {
    ($x:expr, description=$description:expr) => {{
        DumbProgressIterator::new_with_desc($x, $description.to_string())
    }};
    ($x:expr) => {{
        DumbProgressIterator::new($x)
    }};
}
  


pub fn debug_progress(show_items: bool, sleep_millis: u64, level: usize) {
    let items = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("cherry"),
    ];
    let desc = format!("level {}", level);
    // let mut iter = {
    //     let mut builder = DumbProgressIteratorBuilder::new(Box::new(items.iter()));
    //     builder.set_description(desc.as_str());
    //     let mut iter = builder.build();
    //     iter
    // };
    let mut iter = dpiter!(Box::new(items.iter()), description=desc.as_str());
    //let mut iter = { DumbProgressIterator::new_with_desc(Box::new(items.iter()), desc) };
    while let Some(item) = iter.next() {
        if show_items {
            println!("          * iter(): {}", item);
        }
        if sleep_millis > 0 {
            thread::sleep(Duration::from_millis(sleep_millis));
        }
        if level > 0 {
            debug_progress(show_items, sleep_millis, level - 1);
        }
    }
}
pub fn debug_progress_single(show_items: bool, sleep_millis: u64) {
    if true {
        let items = vec![
            String::from("apple"),
            String::from("banana"),
            String::from("cherry"),
        ];
        {
            let mut iter = dpiter!(Box::new(items.iter()));
            //let mut iter = { DumbProgressIterator::new(Box::new(items.iter())) };
            while let Some(item) = iter.next() {
                if show_items {
                    println!("          * iter(): {}", item);
                }
                if sleep_millis > 0 {
                    thread::sleep(Duration::from_millis(sleep_millis));
                }
            }
        }
        if false {
            for item in items.iter() {
                println!("- iter(): {}", item);
            }
        }
    }
    if false {
        let items = vec![
            String::from("apple"),
            String::from("banana"),
            String::from("cherry"),
        ];
        {
            // let mut builder = DumbProgressIteratorBuilder::new(Box::new(items.iter()));
            // //builder.set_description(desc);
            // let mut iter = builder.build();
            //let mut iter = dpiter!(Box::new(items.iter()));
            let mut iter = { DumbProgressIterator::new(Box::new(items.into_iter())) };
            while let Some(item) = iter.next() {
                if show_items {
                    println!("          * into_iter(): {}", item);
                }
                if sleep_millis > 0 {
                    thread::sleep(Duration::from_millis(sleep_millis));
                }
            }
        }
        // for item in items.iter() {
        //     println!("- iter(): {}", item);
        // }
    }

    if false {
        let items = vec!["apple", "banana", "cherry", "date"];
        let mut iter = DPIVector::new(&items);
        while let Some(item) = iter.next() {
            println!("Processing item: {}", item);
        }
    }

    if false {
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

    if false {
        let items = vec!["apple", "banana", "cherry", "date"];
        let mut item_iter = items.iter();
        let mut iter = DPISliceIter::new(&mut item_iter);
        while let Some(item) = iter.next() {
            println!("Processing item: {}", item);
        }
    }
}

// struct DumbProgressIteratorBuilder<'a, T> {
//     boxed_iterator: Option<Box<dyn Iterator<Item = T> + 'a>>,
//     description: Option<String>,
// }

// impl<'a, T> DumbProgressIteratorBuilder<'a, T> {
//     pub fn new(boxed_iterator: Box<dyn Iterator<Item = T> + 'a>) -> DumbProgressIteratorBuilder<T> {
//         DumbProgressIteratorBuilder { 
//             boxed_iterator: Some(boxed_iterator),
//             description: None
//         }
//     }
//     pub fn set_description(&mut self, description: &str) -> &mut Self {
//         self.description = Some(description.to_string());
//         self
//     }
//     pub fn build(&mut self) -> DumbProgressIterator<T> {
//         let progress_entry_id = get_the_progress_shower_ref()
//             .borrow_mut()
//             .register_progress(Progress::Counter(0), self.description.take());
//         DumbProgressIterator {
//             boxed_iterator: self.boxed_iterator.take().unwrap(),
//             //description: self.description.take(),
//             progress_entry_id,
//         }
//     }
// }

struct DumbProgressIterator<'a, T> {
    boxed_iterator: Box<dyn Iterator<Item = T> + 'a>,
    //description: Option<String>,
    progress_entry_id: usize,
}

impl<'a, T> DumbProgressIterator<'a, T> {
    pub fn new(boxed_iterator: Box<dyn Iterator<Item = T> + 'a>) -> DumbProgressIterator<T> {
        DumbProgressIterator::_new(boxed_iterator, None)
    }
    fn new_with_desc(boxed_iterator: Box<dyn Iterator<Item = T> + 'a>, description: String) -> DumbProgressIterator<T> {
        DumbProgressIterator::_new(boxed_iterator, Some(description))
    }
    fn _new(boxed_iterator: Box<dyn Iterator<Item = T> + 'a>, description: Option<String>) -> DumbProgressIterator<T> {
        let progress_entry_id = get_the_progress_shower_ref()
            .borrow_mut()
            .register_progress(Progress::Counter(0), description);
        DumbProgressIterator {
            boxed_iterator,
            //description,
            progress_entry_id,
        }
    }
}

impl<'a, T> Iterator for DumbProgressIterator<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.boxed_iterator.as_mut().next();
        if result.is_some() {
            get_the_progress_shower_ref()
                .borrow_mut()
                .advance_progress(self.progress_entry_id);
        } else {
            get_the_progress_shower_ref()
                .borrow_mut()
                .end_progress(self.progress_entry_id);
        }
        result
    }
}

impl<'a, T> Drop for DumbProgressIterator<'a, T> {
    fn drop(&mut self) {
        //println!("DumbProgressIterator dropped");
        get_the_progress_shower_ref()
            .borrow_mut()
            .unregister_progress(self.progress_entry_id);
    }
}

#[derive(Debug, Clone)]
enum Progress {
    Counter(usize),
}

#[derive(Debug, Clone)]
struct ProgressEntry {
    id: usize,
    progress: Progress,
    description: Option<String>,
}

// impl ProgressEntry {
//     fn new(id: usize, progress: Progress) -> ProgressEntry {
//         ProgressEntry { id, progress }
//     }
// }

struct ProgressShower {
    next_entry_id: usize,
    progress_entries: Vec<ProgressEntry>,
    //showing_progress_entry_count: usize,
}
impl ProgressShower {
    fn register_progress(&mut self, progress: Progress, description: Option<String>) -> usize {
        let entry_id = self.next_entry_id;
        let progress_entry = ProgressEntry {
            id: entry_id, 
            progress: progress,
            description: description,
        };
        self.progress_entries.push(progress_entry);
        self.next_entry_id += 1;
        self._show_progress(false);
        entry_id
    }
    fn unregister_progress(&mut self, entry_id: usize) {
        let idx = self
            .progress_entries
            .iter()
            .position(|entry| entry.id == entry_id);
        if let Some(idx) = idx {
            self.progress_entries = self.progress_entries[0..idx].to_vec();
            //self.showing_progress_entry_count = 0;
            //self.show_progress();
        }
    }
    fn advance_progress(&mut self, entry_id: usize) {
        let idx = self
            .progress_entries
            .iter()
            .position(|entry| entry.id == entry_id);
        if let Some(idx) = idx {
            match self.progress_entries[idx].progress {
                Progress::Counter(ref mut counter) => {
                    *counter += 1;
                }
            }
            self._show_progress(false);
        }
    }
    fn end_progress(&mut self, entry_id: usize) {
        let idx = self
            .progress_entries
            .iter()
            .position(|entry| entry.id == entry_id);
        if let Some(idx) = idx {
            self._show_progress(true);
        }
    }
    fn _show_progress(&mut self, last_progress: bool) {
        let progress_entry_count = self.progress_entries.len();
        if progress_entry_count > 0 {
            print!("\r");
            if !last_progress {
                print!("|");
            }
            for i in 0..progress_entry_count {
                let entry = self.progress_entries.get(i).unwrap();
                match entry.progress {
                    Progress::Counter(counter) => {
                        if !last_progress {
                            print!(" {} | ", counter);
                        }
                    }
                }
            }
            if !last_progress {
                let last_entry = self.progress_entries.get(progress_entry_count - 1).unwrap();
                if let Some(description) = &last_entry.description {
                    print!(" {} ", description);
                }
                print!("... \x1B[K");  // clear rest of line
            }
            io::stdout().flush().unwrap();
        }
    }
    fn _old_show_progress(&mut self, last_progress: bool) {
        // for _ in 0..self.showing_progress_entry_count {
        //     print!("\x1B[1A");
        // }
        if last_progress {
            print!("\r");
            io::stdout().flush().unwrap();
        } else {
            for entry in self.progress_entries.iter() {
                match entry.progress {
                    Progress::Counter(counter) => {
                        if true {
                            print!("\r│ {} ", counter);
                            io::stdout().flush().unwrap();
                        } else if true {
                            print!("\r>>> Progress {}: {} ", entry.id, counter);
                            io::stdout().flush().unwrap();
                        } else if true {
                            println!(">>> Progress {}: {}", entry.id, counter);
                            print!("\x1B[1A"); // move up   
                        } else {
                            let indicator = format!(">>> Progress {}: {}", entry.id, counter);
                            print!("{}", indicator);
                            io::stdout().flush().unwrap();
                            for _ in 0..indicator.len() {
                                print!("\x1B[1D");  // move left
                            }
                            io::stdout().flush().unwrap();
                        }
                    }
                }
            }
        }
    }
}

fn get_the_progress_shower_ref() -> &'static RefCell<ProgressShower> {
    // https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
    // Create an uninitialized static
    static mut SINGLETON: MaybeUninit<RefCell<ProgressShower>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();
    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = ProgressShower {
                next_entry_id: 0,
                progress_entries: Vec::new(),
            };
            // Store it to the static var, i.e. initialize it
            SINGLETON.write(RefCell::new(singleton));
        });
        // Now we give out a shared reference to the data, which is safe to use
        // concurrently.
        SINGLETON.assume_init_ref()
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
