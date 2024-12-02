use std::cell::RefCell;

thread_local! {
    static CHAT: RefCell<Vec<String>> = RefCell::new(Vec::new());
}


#[ic_cdk::query]
fn get_chat()-> Vec<String> {
    // let a = String::from("123");
    // let b = a.clone();
    // let b = &a;
    // let c = a;
    CHAT.with(|chat| chat.borrow().clone())
    //COUNTER.with(|counter| (*counter.borrow()).clone());
}

#[ic_cdk::update]
fn add_msg(new_msg: String){
    CHAT.with(|chat| chat.borrow_mut().push(new_msg))
}
// new_msg : String
// nsg.borrow_mut(): &mut String
// *(&mut String)
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
