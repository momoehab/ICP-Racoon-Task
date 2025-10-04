use candid::{CandidType, Deserialize};
use ic_cdk::query;
use ic_cdk::update;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, CandidType, Deserialize)]
struct Item {
    id: usize,
    data: String,
    state: bool,
}
thread_local! {
static MELIST: RefCell<HashMap<String, Vec<Item>>> = RefCell::new(HashMap::new());
}
//static mut melist: HashMap<String, Vec<Item>> = HashMap::new();

#[update]
fn greet(mut name: String) -> String {
    name = name.trim().to_string();
    MELIST.with(|x| {
        let mut y = x.borrow_mut();
        if !y.contains_key(&name) {
            y.insert(name.clone(), Vec::new());
        }
    });

    format!("Hello, {}!", name)
}
#[query]
fn showlist(name: String) -> Vec<Item> {
    MELIST.with(|x| {
        let mut y = x.borrow_mut();
        y.get(&name).cloned().unwrap_or_default()
    })
}

#[update]
fn changestate(name: String, num: usize) {
    MELIST.with(|z| {
        let mut y = z.borrow_mut();
        if let Some(x) = y.get_mut(&name) {
            if let Some(item) = x.iter_mut().find(|i| i.id == num) {
                item.state = !item.state;
            }
        }
    });
}

#[update]
fn addtolist(name: String, word: String) {
    MELIST.with(|z| {
        let mut y = z.borrow_mut();
        if let Some(x) = y.get_mut(&name) {
            let mut id = 1;
            if !x.last().is_none() {
                id = x.last().unwrap().id + 1;
            };

            x.push(Item {
                id: id,
                data: word,
                state: false,
            });
        }
    });
}

#[update]
fn removetolist(name: String, num: usize) {
    MELIST.with(|z| {
        let mut y = z.borrow_mut();
        if let Some(x) = y.get_mut(&name) {
            if let Some(pos) = x.iter().position(|i| i.id == num) {
                x.remove(pos);
            }
        }
    });
}
#[update]
fn removeall(name: String) {
    MELIST.with(|z| {
        let mut y = z.borrow_mut();
        if let Some(x) = y.get_mut(&name) {
            x.clear();
        }
    });
}
#[update]
fn deleteuser(name: String) {
    MELIST.with(|x| {
        let mut y = x.borrow_mut();
        y.remove(&name);
    });
}

ic_cdk::export_candid!();
