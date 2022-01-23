use on_drop::OnDrop;
use std::sync::Arc;
use std::mem::ManuallyDrop;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

#[test]
#[should_panic(expected = "drop 2 times")]
fn test(){
    let droped = Arc::new(AtomicBool::new(false));
    let droped_tmp = droped.clone();
    let drop_item =OnDrop::new(1, move|| droped_tmp.store(true, Relaxed));
    let mut drop_item=ManuallyDrop::new(drop_item);
    unsafe{ ManuallyDrop::drop(&mut drop_item) };
    assert_eq!(droped.load(Relaxed), true);
    unsafe{ ManuallyDrop::drop(&mut drop_item) };
}