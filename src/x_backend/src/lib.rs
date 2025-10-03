use candid::{CandidType, Deserialize};
use ic_cdk::query;
use ic_cdk::update;

#[derive(Clone, CandidType, Deserialize)]
struct List {
    user: String,
    items: Vec<Item>,
}
#[derive(Clone, CandidType, Deserialize)]
struct Item {
    id: usize,
    data: String,
    state: bool,
}

static mut melist: List = List {
    user: String::new(),
    items: Vec::new(),
};

#[query]
fn greet(mut name: String) -> String {
    name = name.trim().to_string();
    unsafe {
        if melist.user != name {
            melist = List {
                user: name.clone(),
                items: Vec::new(),
            };
        } else {
            melist.user = name.clone();
        }
    }
    format!("Hello, {}!", name)
}
#[query]
fn showlist() -> Vec<Item> {
    unsafe { melist.items.clone() }
}

#[update]
fn changestate(num: usize) {
    unsafe {
        if let Some(item) = melist.items.iter_mut().find(|i| i.id == num) {
            item.state = !item.state;
        }
    }
}

#[update]
fn addtolist(word: String) {
    unsafe {
        melist.items.push(Item {
            id: melist.items.len() + 1,
            data: word,
            state: false,
        });
    }
}

#[update]
fn removetolist(num: usize) {
    unsafe {
        if let Some(pos) = melist.items.iter().position(|i| i.id == num) {
            melist.items.remove(pos);
        }
    }
}
#[update]
fn removeall() {
    unsafe {
        melist.items.clear();
    }
}
#[update]
fn deleteuser() {
    unsafe {
        melist.user = "".to_string();
        melist.items.clear();
    }
}

ic_cdk::export_candid!();
