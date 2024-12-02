use std::cell::RefCell;

thread_local! {
    static MSG: RefCell<String> = RefCell::new(String::new());
}


#[ic_cdk::query]
fn get_msg() -> String {
    // let a = String::from("123");
    // let b = a.clone();
    // let b = &a;
    // let c = a;
    MSG.with(|msg| msg.borrow().clone())

    //COUNTER.with(|counter| (*counter.borrow()).clone());
}

#[ic_cdk::update]
fn set_msg(new_msg: String){
    MSG.with(|msg| *msg.borrow_mut() = new_msg);
}
//
// new_msg : String
// nsg.borrow_mut(): &mut String
// *(&mut String)
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
