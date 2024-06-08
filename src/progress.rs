//! A simple [`Iterator`] wrapper that helps to show progress of iteration -- [`crate::progress::DumbProgressIndicator`]

#![deny(warnings)]
#![allow(unused)]

use std::{
    borrow::Borrow,
    cell::RefCell,
    cmp::min,
    io::{self, Write},
    mem::MaybeUninit,
    ops::Range,
    sync::Once,
    thread,
    time::Duration,
};

/// use this macro to wrap an [Iterator] with [DumbProgressIndicator] to show progress of iteration
///
/// e.g.
/// ```
/// use rusty_dumb_tools::prelude::*;
/// let v = vec![1, 2, 3, 4, 5];
/// for i in dpiw!(v.iter(), name="name", desc="long desc") {
/// }
/// ```
/// Note that `name` and `desc` are optional parameters to [crate::dpiw!]
/// Also see: [crate::dpir!], [crate::dpi_iter!] and [crate::dpi_into_iter!],
#[macro_export]
macro_rules! dpiw {
    ($x:expr
        $(, name=$name:expr)?
        $(, desc=$desc:expr)?
        $(, total=$total:expr)?
    ) => {{
        let mut setting = DumbProgressSetting {
            ..DumbProgressSetting::default()
        };
        $(setting.total = Some($total);)?
        $(setting.name = Some($name.to_string());)?
        $(setting.desc = Some($desc.to_string());)?
        DumbProgressIndicator::new(Box::new($x), setting)
    }};
}

/// use this macro to wrap a [Range] with [DumbProgressIndicator] to show progress of iteration
///
/// e.g.
/// ```
/// use rusty_dumb_tools::prelude::*;
/// for i in dpir!(0..6, name="name", desc="long desc") {
/// }
/// ```
/// Note that `name` and `desc` are optional parameters to [crate::dpir!]
/// Also see: [crate::dpiw!], [crate::dpi_iter!] and [crate::dpi_into_iter!],
#[macro_export]
macro_rules! dpir {
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
        DumbProgressIndicator::new(Box::new($x), setting)
    }};
}

/// use this macro to wrap a [Vec] `iter()` with [DumbProgressIndicator] to show progress of iteration
///
/// e.g.
/// ```
/// use rusty_dumb_tools::prelude::*;
/// let v = vec![1, 2, 3, 4, 5];
/// for i in dpi_iter!(v, name="name", desc="long desc") {
/// }
/// ```
/// Note that `name` and `desc` are optional parameters to [crate::dpi_iter!]
/// Also see: [crate::dpiw!], [crate::dpir!] and [crate::dpi_into_iter!],
#[macro_export] // the same as dpiw, but provide total automatically
macro_rules! dpi_iter {
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
        DumbProgressIndicator::new(Box::new($x.iter()), setting)
    }};
}

/// use this macro to wrap a [Vec] `into_iter()` with [DumbProgressIndicator] to show progress of iteration
///
/// e.g.
/// ```
/// use rusty_dumb_tools::prelude::*;
/// let v = vec![1, 2, 3, 4, 5];
/// for i in dpi_into_iter!(v, name="name", desc="long desc") {
/// }
/// ```
/// Note that `name` and `desc` are optional parameters to [crate::dpi_into_iter!]
/// Also see: [crate::dpiw!], [crate::dpir!] and [crate::dpi_iter!],
#[macro_export] // the same as dpir
macro_rules! dpi_into_iter {
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
        DumbProgressIndicator::new(Box::new($x.into_iter()), setting)
    }};
}

// pub fn debug_progress(show_items: bool, sleep_millis: u64, level: usize) {
//     let items = vec![
//         String::from("apple"),
//         String::from("banana"),
//         String::from("cherry"),
//     ];
//     let desc = format!("level {}", level);
//     // let mut iter = {
//     //     let mut builder = DumbProgressIndicatorBuilder::new(Box::new(items.iter()));
//     //     builder.set_description(desc.as_str());
//     //     let mut iter = builder.build();
//     //     iter
//     // };
//     let name = format!("L{}", level);
//     let mut iter = dpi_into_iter!(items, name = name, desc = desc);
//     //let source = DumbProgressSource::new(Box::new(items.into_iter()));
//     //let mut iter = { DumbProgressIndicator::new_with_desc(source, desc) };
//     while let Some(item) = iter.next() {
//         if show_items {
//             println!("          * iter(): {}", item);
//         }
//         if sleep_millis > 0 {
//             thread::sleep(Duration::from_millis(sleep_millis));
//         }
//         if level > 0 {
//             debug_progress(show_items, sleep_millis, level - 1);
//         }
//     }
//     // if true {
//     //     for item in items.iter() {
//     //         println!("- iter(): {}", item);
//     //     }
//     // }
// }
// pub fn debug_progress_single(show_items: bool, sleep_millis: u64) {
//     if true {
//         let items = vec![
//             String::from("apple"),
//             String::from("banana"),
//             String::from("cherry"),
//         ];
//         {
//             let mut iter = dpiw!(items.iter());
//             //let progress_source = items.to_progress_source();
//             //let mut iter = { DumbProgressIndicator::new(progress_source, DumbProgressSetting::default()) };
//             while let Some(item) = iter.next() {
//                 if show_items {
//                     println!("          * iter(): {}", item);
//                 }
//                 if sleep_millis > 0 {
//                     thread::sleep(Duration::from_millis(sleep_millis));
//                 }
//             }
//         }
//         if true {
//             for item in items.iter() {
//                 println!("- iter(): {}", item);
//             }
//         }
//     }
// }

const DEF_SHOW_GAP_MILLIS: u16 = 100;
const DEF_PREFER_EMOJIS: bool = true;
const MAX_NESTED_PROGRESS_BAR_COUNT: u8 = 2;

/// [DumbProgressIndicator] is a simple [`Iterator`] wrapper that helps to show progress of iteration.
/// It can show the progress of iteration like:
/// ```_no_run
/// 💠 STAGE: 1/3 🌑🌓🌕🌕🌕🌕🌕🌕🌕🌕 – iteration of stages 💠 …  done stage 1
/// 💠 STAGE: 2/3 🌑🌑🌑🌕🌕🌕🌕🌕🌕🌕 – iteration of stages 💠 …  done stage 2
/// 💠 STAGE: 3/3 🌑🌑🌑🌑🌑🌕🌕🌕🌕🌕 – iteration of stages 💠 …  done stage 3
/// ```
/// - `STAGE` -- the name of the progress indicator provided
/// - `1/3` -- `1` means the 1st iteration, and `3` is the total number of iterations
/// - `🌑🌓🌕🌕🌕🌕🌕🌕🌕🌕` -- the progress bar / percentage bar
/// - `iteration of stages` -- the description of the progress indicator provided
/// - `done stage 1` -- the [println!] output of the program
///
/// Although [DumbProgressIndicator] can be created directly, it is recommended to use macro [dpiw!], [dpir!], [dpi_iter!] or [dpi_into_iter!].
/// - For [Range], use [dpir!] like
/// ```
/// use rusty_dumb_tools::prelude::*;
/// for i in dpir!(0..6) {
///     println!("i: {}", i);
/// }
/// ```
/// - For [Vec] `iter()`, use [dpi_iter!] like
/// ```
/// use rusty_dumb_tools::prelude::*;
/// let items = vec![1, 2, 3, 4, 5];
/// for i in dpi_iter!(items) {
///    println!("i: {}", i);
/// }
/// ```
/// - For [Vec] `into_iter()`, use [dpi_into_iter!] like
/// ```
/// use rusty_dumb_tools::prelude::*;
/// let items = vec![1, 2, 3, 4, 5];
/// for i in dpi_into_iter!(items) {
///    println!("i: {}", i);
/// }
/// ```
/// - For open-ended [Range] or explicit [Iterator], use [dpiw!] like
/// ```
/// use rusty_dumb_tools::prelude::*;
/// for i in dpiw!(0..) {
///     let items = vec![1, 2, 3, 4, 5];
///     for item in dpiw!(items.iter()) {
///         println!("i: {}; item: {}", i, item);
///     }
///     if i > 3 {
///         break;
///     }
/// }
/// ```
///
///  In case direct creation is desired, an instance of [DumbProgressIndicator] can be created like:
/// ```
/// use rusty_dumb_tools::prelude::*;
/// let items = vec![1, 2, 3, 4, 5];
/// let mut setting = DumbProgressSetting {
///   total: Some(items.len()),
///   name: Some(String::from("name")),
///   ..DumbProgressSetting::default()
/// };
/// let iter = DumbProgressIndicator::new(Box::new(items.iter()), setting);
/// for i in iter {
/// }
/// ```
///
/// There are additional *global* options that you can set like with
/// * [DumbProgressSetting::set_style] -- set the style for show the progress -- `DumbProgressStyle::Simple` or `DumbProgressStyle::Default`
/// * [DumbProgressSetting::set_max_nested_progress_bar_count] -- in case of nested iteration, set the maximum number of progress bars (percentage bars) to show
pub struct DumbProgressIndicator<'a, T> {
    boxed_iterator: Box<dyn Iterator<Item = T> + 'a>,
    //description: Option<String>,
    progress_entry_id: usize,
}

impl<'a, T> DumbProgressIndicator<'a, T> {
    pub fn new_simple(
        boxed_iterator: Box<dyn Iterator<Item = T> + 'a>,
    ) -> DumbProgressIndicator<T> {
        DumbProgressIndicator::new_ex(boxed_iterator, DumbProgressSetting::default())
    }
    pub fn new(
        boxed_iterator: Box<dyn Iterator<Item = T> + 'a>,
        setting: DumbProgressSetting,
    ) -> DumbProgressIndicator<T> {
        DumbProgressIndicator::new_ex(boxed_iterator, setting)
    }
    fn new_with_desc(
        boxed_iterator: Box<dyn Iterator<Item = T> + 'a>,
        desc: String,
    ) -> DumbProgressIndicator<T> {
        let setting = DumbProgressSetting {
            desc: Some(desc),
            ..DumbProgressSetting::default()
        };
        DumbProgressIndicator::new_ex(boxed_iterator, setting)
    }
    fn new_ex(
        boxed_iterator: Box<dyn Iterator<Item = T> + 'a>,
        setting: DumbProgressSetting,
    ) -> DumbProgressIndicator<T> {
        if false {
            let (start, end) = boxed_iterator.as_ref().size_hint();
            if let Some(e) = end {
                let t = e - start; // TODO: make use of it instead of need to input total
                if let Some(total) = setting.total {
                    assert_eq!(total, t);
                }
            }
        }
        let progress_entry_id = get_the_progress_shower_ref()
            .borrow_mut()
            .register_progress(Progress::Counter(0), setting);
        DumbProgressIndicator {
            boxed_iterator,
            //description,
            progress_entry_id,
        }
    }
}

impl<'a, T> Iterator for DumbProgressIndicator<'a, T> {
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

impl<'a, T> Drop for DumbProgressIndicator<'a, T> {
    fn drop(&mut self) {
        //println!("DumbProgressIndicator dropped");
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
pub enum DumbProgressStyle {
    Simple,
    Default,
}

#[derive(Debug, Clone)]
pub struct DumbProgressSetting {
    pub total: Option<usize>,
    pub name: Option<String>,
    pub desc: Option<String>,
}

impl DumbProgressSetting {
    /// set the style for show the progress -- `DumbProgressStyle::Simple` or `DumbProgressStyle::Default`
    pub fn set_style(style: DumbProgressStyle) {
        get_the_progress_shower_ref().borrow_mut().set_style(style);
    }
    /// in case of nested iteration, set the maximum number of progress bars (percentage bars) to show;
    /// you can set it to 0 to disable showing percentage bar; the default is 2  
    pub fn set_max_nested_progress_bar_count(count: u8) {
        get_the_progress_shower_ref()
            .borrow_mut()
            .set_max_nested_progress_bar_count(count);
    }
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
    show_gap_millis: u16,
    prefer_emojis: bool,
    max_nested_progress_bar_count: u8,
    progress_entries: Vec<ProgressEntry>,
    last_shown_entry_count: Option<usize>,
    last_shown_time: Option<std::time::Instant>,
}
impl ProgressShower {
    fn set_style(&mut self, style: DumbProgressStyle) {
        self.prefer_emojis = match style {
            DumbProgressStyle::Simple => false,
            DumbProgressStyle::Default => true,
        };
    }
    fn set_max_nested_progress_bar_count(&mut self, count: u8) {
        self.max_nested_progress_bar_count = count;
    }
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
                let prefer_emojis = self.prefer_emojis;
                print!("\r");
                if progress_entry_count > 0 {
                    let divider = if prefer_emojis {
                        "💠" //💠 "🔹" //"➕"//"🚪"
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
                                                    - self.max_nested_progress_bar_count as i32;
                                            print!("/{}", total);
                                            let mut percent = counter as f32 / total as f32 * 100.0;
                                            if percent > 100.0 {
                                                percent = 100.0;
                                            }
                                            if graphical_progress {
                                                print!(" ");
                                                let (filled_dot, half_filled_dot, empty_dot) =
                                                    if prefer_emojis {
                                                        if false {
                                                            ("🟦", "🟦", "⬜") // ⏹️⏺️⏺️
                                                        } else if true {
                                                            ("🌑", "🌓", "🌕")
                                                        } else {
                                                            ("🟢", "🟢", "⚪")
                                                        }
                                                    } else {
                                                        ("■", "■", "□") // ◧▣
                                                    };
                                                if true {
                                                    let filled_count = percent as usize / 10;
                                                    let half_filled_count = if (percent
                                                        - (10.0 * filled_count as f32)
                                                        > 5.0)
                                                    {
                                                        1
                                                    } else {
                                                        0
                                                    };
                                                    let empty_count =
                                                        10 - filled_count - half_filled_count;
                                                    print!(
                                                        "{}{}{}",
                                                        filled_dot.repeat(filled_count),
                                                        half_filled_dot.repeat(half_filled_count),
                                                        empty_dot.repeat(empty_count)
                                                    );
                                                } else {
                                                    let dot_count =
                                                        (percent / 10.0).round() as usize;
                                                    print!(
                                                        "{}{}",
                                                        filled_dot.repeat(dot_count),
                                                        empty_dot.repeat(10 - dot_count)
                                                    );
                                                }
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
                prefer_emojis: DEF_PREFER_EMOJIS,
                max_nested_progress_bar_count: MAX_NESTED_PROGRESS_BAR_COUNT,
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

//     fn to_iterator(&'a self) -> (Box<dyn Iterator<Item = T> + 'a>, usize);
// }
// impl<'a, T> DumbProgressIteratorProviderTrait<'a, T> for Range<T> {
//     fn to_iterator(&'a self) -> (Box<dyn Iterator<Item = T> + 'a>, usize) {
//         let r: Range<i32> = 0..10;
//         r.len();
//         let ri = r.into_iter();
//         ri.len();
//         let s = *self;
//         let t = s.len();
//         let i = s.into_iter();
//         let b = Box::new(i);
//         (b, t)
//     }
// }
