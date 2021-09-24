use duplicate::duplicate;
use tagged_tree::Tree;

type TestSubject = Tree<usize, Thing>;

#[derive(Eq, PartialEq, Default)]
struct Thing(usize);

#[duplicate(
    traverse_empty_tree       get_iter;
    [traverse_empty_tree]     [iter_depth_first];
    [traverse_empty_tree_mut] [iter_depth_first_mut];
)]
#[test]
#[allow(unused_mut)]
fn traverse_empty_tree() {
    let mut subject = TestSubject::new(Thing(0));
    let mut iter = subject.get_iter();
    assert!(iter.next().is_none());
}

#[duplicate(
    traverse_tree_with_one_generation       iter_depth_first;
    [traverse_tree_with_one_generation]     [iter_depth_first];
    [traverse_tree_with_one_generation_mut] [iter_depth_first_mut];
)]
#[test]
fn traverse_tree_with_one_generation() {
    let mut subject = TestSubject::new(Thing(0));
    subject.add_child(1, Thing(1));
    subject.add_child(2, Thing(2));
    subject.add_child(3, Thing(3));

    let mut iter = subject.iter_depth_first();

    let (_, thing) = iter.next().expect("thing 1");
    assert_eq!(1, thing.0);

    let (_, thing) = iter.next().expect("thing 2");
    assert_eq!(2, thing.0);

    let (_, thing) = iter.next().expect("thing 3");
    assert_eq!(3, thing.0);

    assert!(iter.next().is_none());
}

#[duplicate(
    traverse_tree_with_two_generations       iter_depth_first;
    [traverse_tree_with_two_generations]     [iter_depth_first];
    [traverse_tree_with_two_generations_mut] [iter_depth_first_mut];
)]
#[test]
fn traverse_tree_with_two_generations() {
    use mockall::{automock, Sequence};

    type TestSubject = Tree<usize, MockObject>;

    #[automock]
    trait Object {
        fn to_be_called(&self) {}
    }

    let mut seq = Sequence::new();

    let mut make_mock = || -> MockObject {
        let mut mock = MockObject::new();
        mock.expect_to_be_called()
            .return_const(())
            .times(1)
            .in_sequence(&mut seq);
        mock
    };

    let mut mock = MockObject::new();
    mock.expect_to_be_called().return_const(()).times(0);
    let mut tree = TestSubject::new(mock);

    let mut counter = 0;
    let mut add_subtree = || {
        let subtree = tree.entry(counter).or_insert(make_mock());
        counter += 1;
        subtree.add_child(counter, make_mock());
        counter += 1;
        subtree.add_child(counter, make_mock());
        counter += 1;
        subtree.add_child(counter, make_mock());
        counter += 1;
    };

    add_subtree();
    add_subtree();
    add_subtree();

    for (id, obj) in tree.iter_depth_first() {
        println!("calling object #{}", id);
        obj.to_be_called();
    }
}
