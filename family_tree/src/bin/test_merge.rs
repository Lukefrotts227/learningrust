use family_tree::{FamilyTree, Person};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {

    // Create first family tree
    let mother1 = Person::new("Alice".to_string(), 165.0, 65.0, 45, 'F');
    let father1 = Person::new("Bob".to_string(), 180.0, 85.0, 47, 'M');
    let mut tree1 = FamilyTree::new(Some(Rc::clone(&mother1)), Some(Rc::clone(&father1)));

    let child1 = Person::new("Charlie".to_string(), 150.0, 50.0, 20, 'M');
    tree1.add_child(Rc::clone(&child1), &mother1, &father1);

    // Create second family tree with common person
    let mother2 = Person::new("Diana".to_string(), 160.0, 55.0, 44, 'F');
    let mut tree2 = FamilyTree::new(Some(Rc::clone(&mother2)), Some(Rc::clone(&father1))); // Same father as tree1

    let child2 = Person::new("Eve".to_string(), 155.0, 55.0, 18, 'F');
    tree2.add_child(Rc::clone(&child2), &mother2, &father1);

}
