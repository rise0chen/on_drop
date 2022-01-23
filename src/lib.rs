use core::mem::MaybeUninit;
use core::ops::Deref;

const INIT_CODE: u32 = 0xF47E8A5B;

pub struct OnDrop<T> {
    data: T,
    is_init: u32,
    drop_time: u8,
    on_drop: MaybeUninit<Box<dyn FnOnce()>>,
}
impl<T> OnDrop<T> {
    pub fn new<F: 'static + FnOnce()>(data: T, on_drop: F) -> Self {
        Self {
            data,
            is_init: INIT_CODE,
            drop_time: 0,
            on_drop: MaybeUninit::new(Box::new(on_drop)),
        }
    }
}
impl<T> Drop for OnDrop<T> {
    fn drop(&mut self) {
        if self.is_init == INIT_CODE {
            self.drop_time += 1;
            if self.drop_time != 1 {
                panic!("drop {} times", self.drop_time);
            }
            (unsafe { self.on_drop.as_ptr().read() })();
        } else {
            panic!("uninitialized");
        }
    }
}
impl<T> Deref for OnDrop<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<T: PartialEq> PartialEq for OnDrop<T> {
    fn eq(&self, other: &OnDrop<T>) -> bool {
        self.data == other.data
    }
}
