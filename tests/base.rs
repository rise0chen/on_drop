use on_drop::OnDrop;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

#[test]
fn test(){
    let droped = Arc::new(AtomicBool::new(false));
    let droped_tmp = droped.clone();
    let drop_item =OnDrop::new(1, move|| droped_tmp.store(true, Relaxed));
    drop(drop_item);
    assert_eq!(droped.load(Relaxed), true);
}