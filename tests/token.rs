use on_drop::OnDrop;

#[test]
fn test(){
    let (drop_item,drop_token) =OnDrop::token(1);
    assert_eq!(drop_token.is_droped(), false);
    drop(drop_item);
    assert_eq!(drop_token.is_droped(), true);
}