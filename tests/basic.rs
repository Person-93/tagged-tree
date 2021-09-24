use tagged_tree::*;

#[test]
fn can_make_tree() {
    let subject = TestSubject::new(Thing(0));
    assert_eq!(subject.value(), &Thing(0));
}

#[test]
fn can_make_tree_with_children() {
    let mut subject = TestSubject::new(Thing(0));
    subject.add_child(1, Thing(1));
    subject.add_child(2, Thing(2));
    subject.add_child(3, Thing(3));

    let subject = subject; // remove mutability

    assert_eq!(subject.get_child(&1_usize).unwrap().value(), &Thing(1));
    assert_eq!(subject.get_child(&2_usize).unwrap().value(), &Thing(2));
    assert_eq!(subject.get_child(&3_usize).unwrap().value(), &Thing(3));
}

type TestSubject = Tree<usize, Thing>;

#[derive(Eq, PartialEq, Debug)]
struct Thing(usize);
