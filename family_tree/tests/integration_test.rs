use family_tree::{Person, FamilyTree};
use std::rc::Rc;
use std::cell::RefCell;


#[test]
fn test_family_tree_integration() {
    let mother = Person::new("Alice".to_string(), 165.0, 65.0, 45, 'F');
    let father = Person::new("Bob".to_string(), 180.0, 85.0, 47, 'M');
    let mut family_tree = FamilyTree::new(Some(Rc::clone(&mother)), Some(Rc::clone(&father)));

    let child1 = Person::new("Charlie".to_string(), 150.0, 50.0, 20, 'M');
    let child2 = Person::new("Diana".to_string(), 155.0, 55.0, 18, 'F');

    family_tree.add_child(Rc::clone(&child1), &mother, &father);
    family_tree.add_child(Rc::clone(&child2), &mother, &father);

    assert_eq!(mother.borrow().children.len(), 2);
    assert_eq!(father.borrow().children.len(), 2);

    let siblings_option = family_tree.get_siblings(&child1, 'a');
    match siblings_option {
        Some(siblings) => {
            assert_eq!(siblings.len(), 1);
            assert_eq!(siblings[0].borrow().name, "Diana");
        }
        None => {
            println!("No siblings found for Charlie.");
            assert!(false, "Charlie should have one sibling, Diana.");
        }
    }
}
