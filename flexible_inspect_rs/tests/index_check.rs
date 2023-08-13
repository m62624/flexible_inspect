use flexible_inspect_rs::prelude::*;
use indexmap::IndexSet;
use std::collections::VecDeque;

#[test]
fn cehck() {
    let mut vec_decue_stack: VecDeque<Rule> = VecDeque::new();
    vec_decue_stack.push_back(Rule::new(r"[a-zA-z]+", MatchRequirement::MustBeFound));
    vec_decue_stack.push_back(Rule::new("hello", MatchRequirement::MustBeFound));
    vec_decue_stack.push_back(Rule::new(r"world", MatchRequirement::MustNotBeFound));
    vec_decue_stack.push_back(Rule::new(r"user", MatchRequirement::MustBeFound));
    vec_decue_stack.push_back(Rule::new(r"IOI", MatchRequirement::MustBeFound));
    vec_decue_stack.push_back(Rule::new(r":D", MatchRequirement::MustNotBeFound));

    println!("{:#?}", vec_decue_stack.pop_front());
    // let coll_rules = vec![
    //     Rule::new(r"[a-zA-z]+", MatchRequirement::MustBeFound).extend([
    //         Rule::new("hello", MatchRequirement::MustBeFound),
    //         Rule::new(r"world", MatchRequirement::MustNotBeFound),
    //         Rule::new(r"user", MatchRequirement::MustBeFound)
    //             .extend([Rule::new(r"IOI", MatchRequirement::MustBeFound)]),
    //         Rule::new(r":D", MatchRequirement::MustNotBeFound),
    //     ]),
    // ];
    let mut index_set = IndexSet::new();
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    index_set.insert(4);
    println!("{:#?}", index_set);
    index_set.remove(&3);
    println!("{:#?}", index_set);
}
