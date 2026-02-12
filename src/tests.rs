use crate::EpochResetArray;

#[test]
fn test_basic() {
    let mut array = EpochResetArray::<_, _, u8>::new(1, 5);
    assert_eq!(array.get(0), &1);
    assert_eq!(array.get(4), &1);

    array.set(2, 42);
    assert_eq!(array.get(2), &42);
    array.reset();
    assert_eq!(array.get(2), &1);
}

#[test]
fn test_full_reset() {
    let mut array = EpochResetArray::<_, _, u8>::new(1, 5);

    array.set(1, 3);

    for _ in 0..=u8::MAX {
        array.reset();
        assert_eq!(array.get(1), &1);
    }

    array.reset();
    assert_eq!(array.get(1), &1);
}
