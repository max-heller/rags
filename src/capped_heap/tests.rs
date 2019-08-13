use super::CappedHeap;

#[test]
fn zero_cap() {
    let mut heap = CappedHeap::new(0);
    assert!(!heap.insert(1));
    assert!(!heap.insert(2));
    assert_eq!(heap.heap.into_vec(), vec![]);
}

#[test]
fn one_cap() {
    let mut heap = CappedHeap::new(1);
    assert!(heap.insert(1));
    assert!(heap.insert(2));
    assert_eq!(heap.heap.into_vec(), vec![2]);
}

#[test]
fn two_cap() {
    let mut heap = CappedHeap::new(2);
    assert!(heap.insert(1));
    assert!(heap.insert(3));
    assert!(heap.insert(2));
    assert!(!heap.insert(1));
    assert_eq!(heap.heap.into_vec_desc(), vec![3, 2]);
}

#[test]
fn under_cap() {
    let mut heap = CappedHeap::new(3);
    assert!(heap.insert(2));
    assert!(heap.insert(1));
    assert_eq!(heap.heap.into_vec_desc(), vec![2, 1]);
}