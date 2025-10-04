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
static melist: RefCell<HashMap<String, Vec<Item>>> = RefCell::new(HashMap::new());
}
//static mut melist: HashMap<String, Vec<Item>> = HashMap::new();

#[query]
fn greet(mut name: String) -> String {
    name = name.trim().to_string();
    unsafe {
        if !melist.contains_key(&name) {
            melist.insert(name.clone(), Vec::new());
        }
    }
    format!("Hello, {}!", name)
}
#[query]
fn showlist(name: String) -> Vec<Item> {
    unsafe { melist.get(&name).cloned().unwrap_or_default() }
}

#[update]
fn changestate(name: String, num: usize) {
    unsafe {
        if let Some(x) = melist.get_mut(&name) {
            if let Some(item) = x.iter_mut().find(|i| i.id == num) {
                item.state = !item.state;
            }
        }
    }
}

#[update]
fn addtolist(name: String, word: String) {
    unsafe {
        if let Some(x) = melist.get_mut(&name) {
            x.push(Item {
                id: x.len() + 1,
                data: word,
                state: false,
            });
        }
    }
}

#[update]
fn removetolist(name: String, num: usize) {
    unsafe {
        if let Some(x) = melist.get_mut(&name) {
            if let Some(pos) = x.iter().position(|i| i.id == num) {
                x.remove(pos);
            }
        }
    }
}
#[update]
fn removeall(name: String) {
    unsafe {
        if let Some(x) = melist.get_mut(&name) {
            x.clear();
        }
    }
}
#[update]
fn deleteuser(name: String) {
    unsafe {
        melist.remove(&name);
    }
}

ic_cdk::export_candid!();
