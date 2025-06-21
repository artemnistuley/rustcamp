use std::sync::{Arc, Mutex};
use std::thread;
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::marker::PhantomData;

struct OnlySync {
    _marker: PhantomData<*const u8>,
    data: i32,
}

unsafe impl Sync for OnlySync {}

impl OnlySync {
    fn new(value: i32) -> Self {
        Self {
            _marker: PhantomData,
            data: value,
        }
    }

    fn get(&self) -> i32 {
        self.data
    }
}

struct OnlySend {
    data: Cell<i32>,
}

impl OnlySend {
    fn new(value: i32) -> Self {
        Self {
            data: Cell::new(value),
        }
    }

    fn get(&self) -> i32 {
        self.data.get()
    }

    fn set(&self, value: i32) {
        self.data.set(value);
    }
}

struct SyncAndSend {
    data: Arc<Mutex<i32>>,
}

impl SyncAndSend {
    fn new (value: i32) -> Self {
        Self {
            data: Arc::new(Mutex::new(value)),
        }
    }

    fn get(&self) -> i32 {
        *self.data.lock().unwrap()
    }

    fn set(&self, value: i32) {
        *self.data.lock().unwrap() = value;
    }
}

struct NotSyncNotSend {
    data: Rc<RefCell<i32>>,
}

impl NotSyncNotSend {
    fn new(value: i32) -> Self {
        Self {
            data: Rc::new(RefCell::new(value)),
        }
    }

    fn get(&self) -> i32 {
        *self.data.borrow()
    }

    fn set(&self, value: i32)  {
        *self.data.borrow_mut() = value;
    }
}

fn main() {

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    let only_sync: OnlySync = OnlySync::new(42);
    assert_sync::<OnlySync>();
    // assert_send::<OnlySync>();

    let only_send = OnlySend::new(100);
    assert_send::<OnlySend>();
    // assert_sync::<OnlySend>(); 

    let sync_and_send = SyncAndSend::new(100);
    assert_send::<SyncAndSend>();
    assert_sync::<SyncAndSend>();
    
    let not_sync_not_send = NotSyncNotSend::new(100);
    // assert_send::<NotSyncNotSend>();
    // assert_sync::<NotSyncNotSend>();
}
