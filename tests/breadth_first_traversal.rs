use duplicate::duplicate;
use tagged_tree::Tree;

type TestSubject = Tree<usize, Thing>;

#[derive(Eq, PartialEq, Default)]
struct Thing(usize);

#[duplicate(
    traverse_empty_tree       iter_breadth_first;
    [traverse_empty_tree]     [iter_breadth_first];
    [traverse_empty_tree_mut] [iter_breadth_first_mut];
)]
#[test]
#[allow(unused_mut)]
fn traverse_empty_tree() {
    let mut subject = TestSubject::new(Thing(0));
    let mut iter = subject.iter_breadth_first();
    assert!(iter.next().is_none());
}

#[duplicate(
    traverse_tree_with_one_generation       iter_breadth_first;
    [traverse_tree_with_one_generation]     [iter_breadth_first];
    [traverse_tree_with_one_generation_mut] [iter_breadth_first_mut];
)]
#[test]
fn traverse_tree_with_one_generation() {
    let mut subject = TestSubject::new(Thing(0));
    subject.add_child(1, Thing(1));
    subject.add_child(2, Thing(2));
    subject.add_child(3, Thing(3));

    let mut iter = subject.iter_breadth_first();

    let (_, thing) = iter.next().expect("thing 1");
    assert_eq!(1, thing.0);

    let (_, thing) = iter.next().expect("thing 2");
    assert_eq!(2, thing.0);

    let (_, thing) = iter.next().expect("thing 3");
    assert_eq!(3, thing.0);

    assert!(iter.next().is_none());
}

#[duplicate(
    traverse_tree_with_two_generations       iter_breadth_first;
    [traverse_tree_with_two_generations]     [iter_breadth_first];
    [traverse_tree_with_two_generations_mut] [iter_breadth_first_mut];
)]
#[test]
fn traverse_tree_with_two_generations() {
    let mut tree = TestSubject::new(Thing(0));

    let child = tree.entry(1).or_insert(Thing(1));
    child.add_child(4, Thing(4));
    child.add_child(5, Thing(5));
    child.add_child(6, Thing(6));

    let child = tree.entry(2).or_insert(Thing(2));
    child.add_child(7, Thing(7));
    child.add_child(8, Thing(8));
    child.add_child(9, Thing(9));

    let child = tree.entry(3).or_insert(Thing(3));
    child.add_child(10, Thing(10));
    child.add_child(11, Thing(11));
    child.add_child(12, Thing(12));

    let mut counter = 0_usize;
    for (_, Thing(current)) in tree.iter_breadth_first() {
        counter += 1;
        assert_eq!(current, &counter);
    }
}
