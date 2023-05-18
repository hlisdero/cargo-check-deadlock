//! Central structure to keep track of the threads in the code.
//!
//! The `ThreadManager` stores the threads discovered so far in the code.
//! It also performs the translation for each thread function.
//!
//! Once the translation of the main thread is over, each thread stored
//! here will be translated in order.

use super::thread::Thread;
use super::{CondvarRef, MutexGuardRef, MutexRef, ThreadRef};
use crate::data_structures::petri_net_interface::TransitionRef;
use crate::translator::mir_function::Entries;
use log::info;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct ThreadManager {
    threads: Vec<Rc<RefCell<Thread>>>,
}

impl ThreadManager {
    /// Adds a new thread and returns a reference to it.
    pub fn add_thread(
        &mut self,
        spawn_transition: TransitionRef,
        thread_function_def_id: rustc_hir::def_id::DefId,
        memory_entries: (
            Entries<MutexRef>,
            Entries<MutexGuardRef>,
            Entries<ThreadRef>,
            Entries<CondvarRef>,
        ),
    ) -> ThreadRef {
        let index = self.threads.len();
        let thread = Rc::new(RefCell::new(Thread::new(
            spawn_transition,
            thread_function_def_id,
            memory_entries.0,
            memory_entries.1,
            memory_entries.2,
            memory_entries.3,
            index,
        )));
        let thread_ref = Rc::clone(&thread);
        self.threads.push(thread);
        info!("Found thread {index} and pushed it to the back of the thread translation queue");
        thread_ref
    }

    /// Returns the vector of threads found.
    /// After this call, the `ThreadManager` cannot be used anymore.
    pub fn get_threads_found(&mut self) -> Vec<Rc<RefCell<Thread>>> {
        std::mem::take(&mut self.threads)
    }
}
