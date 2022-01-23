use on_drop::OnDrop;
use std::mem::MaybeUninit;

#[test]
#[should_panic(expected = "uninitialized")]
fn test() {
    let drop_item: MaybeUninit<OnDrop<i32>> = MaybeUninit::uninit();
    drop(unsafe { drop_item.assume_init() });
}
