pub mod threads;
pub mod thread_data_transfer;
pub mod thread_shared_state;
pub mod thread_send_sync;

fn main() {
    threads::run();
    println!("+++++++++++++++++++");
    thread_data_transfer::run();
    println!("+++++++++++++++++++");
    thread_shared_state::run();
    println!("+++++++++++++++++++");
    thread_send_sync::run();
}
