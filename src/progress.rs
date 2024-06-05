#![deny(warnings)]
#![allow(unused)]

use std::{
    cell::RefCell,
    io::{self, Write},
    mem::MaybeUninit,
    sync::Once,
    thread,
    time::Duration,
};

#[macro_export]
macro_rules! dpiter {
    ($x:expr
        $(, name=$name:expr)?
        $(, desc=$desc:expr)?
    ) => {{
        let mut setting = DumbProgressSetting {
            ..DumbProgressSetting::default()
        };
        setting.total = Some($x.len());
        $(setting.name = Some($name.to_string());)?
        $(setting.desc = Some($desc.to_string());)?
        DumbProgressIterator::new(Box::new($x.iter()), setting)
    }};
}
#[macro_export]
macro_rules! dpiter_nt {
    ($x:expr
        $(, name=$name:expr)?
        $(, desc=$desc:expr)?
    ) => {{
        let mut setting = DumbProgressSetting {
            ..DumbProgressSetting::default()
        };
        setting.total = None;
        $(setting.name = Some($name.to_string());)?
        $(setting.desc = Some($desc.to_string());)?
        DumbProgressIterator::new(Box::new($x.iter()), setting)
    }};
}

#[macro_export]
macro_rules! dpintoiter {
    ($x:expr
        $(, name=$name:expr)?
        $(, desc=$desc:expr)?
    ) => {{
        let mut setting = DumbProgressSetting {
            ..DumbProgressSetting::default()
        };
        setting.total = Some($x.len());
        $(setting.name = Some($name.to_string());)?
        $(setting.desc = Some($desc.to_string());)?
        DumbProgressIterator::new(Box::new($x.into_iter()), setting)
    }};
}
#[macro_export]
macro_rules! dpintoiter_nt {
    ($x:expr
        $(, name=$name:expr)?
        $(, desc=$desc:expr)?
    ) => {{
        let mut setting = DumbProgressSetting {
            ..DumbProgressSetting::default()
        };
        setting.total = None;
        $(setting.name = Some($name.to_string());)?
        $(setting.desc = Some($desc.to_string());)?
        DumbProgressIterator::new(Box::new($x.into_iter()), setting)
    }};
}

#[macro_export]
macro_rules! dprange {
    ($x:expr
        $(, name=$name:expr)?
        $(, desc=$desc:expr)?
    ) => {{
        let mut setting = DumbProgressSetting {
            ..DumbProgressSetting::default()
        };
        setting.total = Some($x.len());
        $(setting.name = Some($name.to_string());)?
        $(setting.desc = Some($desc.to_string());)?
        DumbProgressIterator::new(Box::new($x.into_iter()), setting)
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
    let name = format!("L{}", level);
    let mut iter = dpintoiter!(items, name = name, desc = desc);
    //let source = DumbProgressSource::new(Box::new(items.into_iter()));
    //let mut iter = { DumbProgressIterator::new_with_desc(source, desc) };
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
    // if true {
    //     for item in items.iter() {
    //         println!("- iter(): {}", item);
    //     }
    // }
}
pub fn debug_progress_single(show_items: bool, sleep_millis: u64) {
    if true {
        let items = vec![
            String::from("apple"),
            String::from("banana"),
            String::from("cherry"),
        ];
        {
            let mut iter = dpiter!(items);
            //let progress_source = items.to_progress_source();
            //let mut iter = { DumbProgressIterator::new(progress_source, DumbProgressSetting::default()) };
            while let Some(item) = iter.next() {
                if show_items {
                    println!("          * iter(): {}", item);
                }
                if sleep_millis > 0 {
                    thread::sleep(Duration::from_millis(sleep_millis));
                }
            }
        }
        if true {
            for item in items.iter() {
                println!("- iter(): {}", item);
            }
        }
    }
}

const DEF_SHOW_GAP_MILLIS: u32 = 100;
const PREFER_EMOJIS: bool = false;
const MAX_PROGRESS_BAR_COUNT: i32 = 3;

pub struct DumbProgressIterator<'a, T> {
    boxed_iterator: Box<dyn Iterator<Item = T> + 'a>,
    //description: Option<String>,
    progress_entry_id: usize,
}

impl<'a, T> DumbProgressIterator<'a, T> {
    pub fn new_simple(boxed_iterator: Box<dyn Iterator<Item = T> + 'a>) -> DumbProgressIterator<T> {
        DumbProgressIterator::new_ex(boxed_iterator, DumbProgressSetting::default())
    }
    pub fn new(
        boxed_iterator: Box<dyn Iterator<Item = T> + 'a>,
        setting: DumbProgressSetting,
    ) -> DumbProgressIterator<T> {
        DumbProgressIterator::new_ex(boxed_iterator, setting)
    }
    fn new_with_desc(
        boxed_iterator: Box<dyn Iterator<Item = T> + 'a>,
        desc: String,
    ) -> DumbProgressIterator<T> {
        let setting = DumbProgressSetting {
            desc: Some(desc),
            ..DumbProgressSetting::default()
        };
        DumbProgressIterator::new_ex(boxed_iterator, setting)
    }
    fn new_ex(
        boxed_iterator: Box<dyn Iterator<Item = T> + 'a>,
        setting: DumbProgressSetting,
    ) -> DumbProgressIterator<T> {
        let progress_entry_id = get_the_progress_shower_ref()
            .borrow_mut()
            .register_progress(Progress::Counter(0), setting);
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

// pub struct DumbProgressSource<'a, T> {
//     boxed_iterator: Box<dyn Iterator<Item = T> + 'a>,
// }

// impl<'a, T> DumbProgressSource<'a, T> {
//     pub fn new(boxed_iterator: Box<dyn Iterator<Item = T> + 'a>) -> DumbProgressSource<'a, T> {
//         DumbProgressSource {
//             boxed_iterator: boxed_iterator,
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct DumbProgressSetting {
    pub total: Option<usize>,
    pub name: Option<String>,
    pub desc: Option<String>,
}

impl Default for DumbProgressSetting {
    fn default() -> Self {
        Self {
            total: None,
            name: None,
            desc: None,
        }
    }
}

#[derive(Debug, Clone)]
enum Progress {
    Counter(usize),
}

#[derive(Debug, Clone)]
struct ProgressEntry {
    entry_id: usize,
    progress: Progress,
    setting: DumbProgressSetting,
}

impl ProgressEntry {
    fn new(entry_id: usize, progress: Progress, setting: DumbProgressSetting) -> ProgressEntry {
        ProgressEntry {
            entry_id,
            progress,
            setting,
        }
    }
}

struct ProgressShower {
    next_entry_id: usize,
    show_gap_millis: u32,
    progress_entries: Vec<ProgressEntry>,
    last_shown_entry_count: Option<usize>,
    last_shown_time: Option<std::time::Instant>,
}
impl ProgressShower {
    fn register_progress(&mut self, progress: Progress, setting: DumbProgressSetting) -> usize {
        let entry_id = self.next_entry_id;
        let progress_entry = ProgressEntry {
            entry_id,
            progress,
            setting,
        };
        self.progress_entries.push(progress_entry);
        self.next_entry_id += 1;
        self._show_progress(false, false);
        entry_id
    }
    fn unregister_progress(&mut self, entry_id: usize) {
        let idx = self
            .progress_entries
            .iter()
            .position(|entry| entry.entry_id == entry_id);
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
            .position(|entry| entry.entry_id == entry_id);
        if let Some(idx) = idx {
            match self.progress_entries[idx].progress {
                Progress::Counter(ref mut counter) => {
                    *counter += 1;
                }
            }
            self._show_progress(true, false);
        }
    }
    fn end_progress(&mut self, entry_id: usize) {
        let idx = self
            .progress_entries
            .iter()
            .position(|entry| entry.entry_id == entry_id);
        if let Some(idx) = idx {
            self._show_progress(false, true);
        }
    }
    fn _show_progress(&mut self, track_timing: bool, last_progress: bool) {
        let progress_entry_count = self.progress_entries.len();
        if progress_entry_count > 0 {
            let mut show_it = last_progress;
            if !show_it {
                if let Some(last_shown_entry_count) = self.last_shown_entry_count {
                    if last_shown_entry_count != progress_entry_count {
                        show_it = true;
                    }
                }
            }
            if !show_it {
                if let Some(last_shown_time) = self.last_shown_time {
                    let now: std::time::Instant = std::time::Instant::now();
                    let millis_form_last = now.duration_since(last_shown_time).as_millis();
                    if millis_form_last >= self.show_gap_millis as u128 {
                        show_it = true;
                    }
                } else {
                    show_it = true;
                }
            }
            //show_it = true;  // just for testing
            if show_it {
                print!("\r");
                if progress_entry_count > 0 {
                    let divider = if PREFER_EMOJIS {
                        "💠" //"🔹" //"➕"//"🚪"
                    } else {
                        "|"
                    };
                    for i in 0..progress_entry_count {
                        let entry = self.progress_entries.get(i).unwrap();
                        match entry.progress {
                            Progress::Counter(counter) => {
                                if !last_progress {
                                    print!("{}", divider);
                                    print!(" ");
                                    if let Some(name) = &entry.setting.name {
                                        print!("{}: ", name);
                                    }
                                    if let Some(total) = entry.setting.total {
                                        if counter <= 9 {
                                            print!("{}", counter);
                                        } else if counter <= 99 {
                                            print!("{:2}", counter);
                                        } else {
                                            print!("{:3}", counter);
                                        }
                                    } else {
                                        print!("{}", counter);
                                    }
                                    if let Some(total) = entry.setting.total {
                                        if total > 0 {
                                            let graphical_progress = i as i32
                                                > progress_entry_count as i32
                                                    - 1
                                                    - MAX_PROGRESS_BAR_COUNT;
                                            print!("/{}", total);
                                            let percent = counter as f32 / total as f32 * 100.0;
                                            if graphical_progress {
                                                print!(" ");
                                                let (filled_dot, half_filled_dot, empty_dot) =
                                                    if PREFER_EMOJIS {
                                                        if true {
                                                            ("🟦", "⏹️", "⬜") // ⏺️
                                                        } else if false {
                                                            ("🌑", "🌓", "🌕")
                                                        } else {
                                                            ("🟢", "🟢", "⚪")
                                                        }
                                                    } else {
                                                        ("■", "■", "□") // ◧▣
                                                    };
                                                let dot_count = (percent / 10.0).round() as usize;
                                                print!(
                                                    "{}{}",
                                                    filled_dot.repeat(dot_count),
                                                    empty_dot.repeat(10 - dot_count)
                                                );
                                            } else {
                                                print!(" ({}%)", percent as i32);
                                            }
                                        }
                                    }
                                    print!(" ");
                                }
                            }
                        }
                    }
                    if !last_progress {
                        let last_entry =
                            self.progress_entries.get(progress_entry_count - 1).unwrap();
                        if let Some(desc) = &last_entry.setting.desc {
                            print!("– {} ", desc);
                        }
                        print!("{} … ", divider);
                    }
                    io::stdout().flush().unwrap();
                }
                if track_timing {
                    self.last_shown_entry_count = Some(progress_entry_count);
                    self.last_shown_time = Some(std::time::Instant::now());
                } else {
                    self.last_shown_entry_count = None;
                    self.last_shown_time = None;
                }
            }
            print!("\x1B[K"); // clear rest of line
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
                            print!("\r>>> Progress {}: {} ", entry.entry_id, counter);
                            io::stdout().flush().unwrap();
                        } else if true {
                            println!(">>> Progress {}: {}", entry.entry_id, counter);
                            print!("\x1B[1A"); // move up
                        } else {
                            let indicator = format!(">>> Progress {}: {}", entry.entry_id, counter);
                            print!("{}", indicator);
                            io::stdout().flush().unwrap();
                            for _ in 0..indicator.len() {
                                print!("\x1B[1D"); // move left
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
                show_gap_millis: DEF_SHOW_GAP_MILLIS,
                last_shown_entry_count: None,
                last_shown_time: None,
            };
            // Store it to the static var, i.e. initialize it
            SINGLETON.write(RefCell::new(singleton));
        });
        // Now we give out a shared reference to the data, which is safe to use
        // concurrently.
        SINGLETON.assume_init_ref()
    }
}

// struct DPIVector<'a, T> {
//     items: &'a Vec<T>,
//     current: usize,
// }

// impl<'a, T> DPIVector<'a, T> {
//     pub fn new(items: &'a Vec<T>) -> DPIVector<'a, T> {
//         DPIVector { items, current: 0 }
//     }
// }

// impl<'a, T> Iterator for DPIVector<'a, T> {
//     type Item = &'a T;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.current < self.items.len() {
//             let item = &self.items[self.current];
//             self.current += 1;
//             Some(item)
//         } else {
//             None
//         }
//     }
// }

// struct DPIIntoIter<'a, T> {
//     iter: &'a mut std::vec::IntoIter<T>,
// }

// impl<'a, T> DPIIntoIter<'a, T> {
//     pub fn new(iter: &'a mut std::vec::IntoIter<T>) -> DPIIntoIter<'a, T> {
//         DPIIntoIter { iter }
//     }
// }

// impl<'a, T> Iterator for DPIIntoIter<'a, T> {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter.next()
//     }
// }

// struct DPISliceIter<'a, T> {
//     iter: &'a mut std::slice::Iter<'a, T>,
// }

// impl<'a, T> DPISliceIter<'a, T> {
//     pub fn new(iter: &'a mut std::slice::Iter<'a, T>) -> DPISliceIter<'a, T> {
//         DPISliceIter { iter }
//     }
// }

// impl<'a, T> Iterator for DPISliceIter<'a, T> {
//     type Item = &'a T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter.next()
//     }
// }

// pub trait DumbProgressSourceRefTrait<'a, T> {
//     fn to_progress_source(&'a self) -> DumbProgressSource<'a, &T>;
//     fn into_progress_source(&'a self) -> DumbProgressSource<'a, &T>;
// }

// // impl<'a, T> DumbProgressProviderTrait<'a, T> for dyn Iterator<Item = &T> + 'a {
// //     fn to_progress_source(&'a self) -> DumbProgressSource<'a, &T> {
// //         let boxed_iterator = Box::new(self);
// //         DumbProgressSource {
// //             boxed_iterator,
// //         }
// //     }
// // }
// impl<'a, T> DumbProgressSourceRefTrait<'a, T> for Vec<T> {
//     fn to_progress_source(&'a self) -> DumbProgressSource<'a, &T> {
//         let boxed_iterator = Box::new(self.iter());
//         DumbProgressSource::new(boxed_iterator)
//     }
//     fn into_progress_source(&'a self) -> DumbProgressSource<'a, &T> {
//         let boxed_iterator = Box::new(self.into_iter());
//         DumbProgressSource::new(boxed_iterator)
//     }
// }
