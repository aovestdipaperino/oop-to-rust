//! Chapter 14: Message Passing - Actor Pattern

use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

// Actor messages
enum CounterMessage {
    Increment,
    Decrement,
    Get(Sender<i64>),
    Reset,
    Stop,
}

struct CounterActor {
    receiver: mpsc::Receiver<CounterMessage>,
    value: i64,
}

impl CounterActor {
    fn new(receiver: mpsc::Receiver<CounterMessage>) -> Self {
        Self { receiver, value: 0 }
    }

    fn run(&mut self) {
        println!("[CounterActor] Started");

        while let Ok(msg) = self.receiver.recv() {
            match msg {
                CounterMessage::Increment => {
                    self.value += 1;
                    println!("[CounterActor] Incremented to {}", self.value);
                }
                CounterMessage::Decrement => {
                    self.value -= 1;
                    println!("[CounterActor] Decremented to {}", self.value);
                }
                CounterMessage::Get(reply_tx) => {
                    let _ = reply_tx.send(self.value);
                }
                CounterMessage::Reset => {
                    self.value = 0;
                    println!("[CounterActor] Reset to 0");
                }
                CounterMessage::Stop => {
                    println!("[CounterActor] Stopping");
                    break;
                }
            }
        }

        println!("[CounterActor] Stopped");
    }
}

// Actor handle for sending messages
#[derive(Clone)]
struct CounterHandle {
    sender: Sender<CounterMessage>,
}

impl CounterHandle {
    fn spawn() -> (Self, JoinHandle<()>) {
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let mut actor = CounterActor::new(rx);
            actor.run();
        });

        (Self { sender: tx }, handle)
    }

    fn increment(&self) {
        let _ = self.sender.send(CounterMessage::Increment);
    }

    fn decrement(&self) {
        let _ = self.sender.send(CounterMessage::Decrement);
    }

    fn get(&self) -> i64 {
        let (tx, rx) = mpsc::channel();
        let _ = self.sender.send(CounterMessage::Get(tx));
        rx.recv().unwrap_or(0)
    }

    fn reset(&self) {
        let _ = self.sender.send(CounterMessage::Reset);
    }

    fn stop(&self) {
        let _ = self.sender.send(CounterMessage::Stop);
    }
}

// Bank account actor example
enum AccountMessage {
    Deposit(u64),
    Withdraw(u64, Sender<Result<(), String>>),
    Balance(Sender<u64>),
    Stop,
}

struct BankAccountActor {
    receiver: mpsc::Receiver<AccountMessage>,
    balance: u64,
    account_id: String,
}

impl BankAccountActor {
    fn new(receiver: mpsc::Receiver<AccountMessage>, account_id: &str) -> Self {
        Self {
            receiver,
            balance: 0,
            account_id: account_id.to_string(),
        }
    }

    fn run(&mut self) {
        println!("[Account {}] Opened", self.account_id);

        while let Ok(msg) = self.receiver.recv() {
            match msg {
                AccountMessage::Deposit(amount) => {
                    self.balance += amount;
                    println!(
                        "[Account {}] Deposited {}, balance: {}",
                        self.account_id, amount, self.balance
                    );
                }
                AccountMessage::Withdraw(amount, reply_tx) => {
                    if amount <= self.balance {
                        self.balance -= amount;
                        println!(
                            "[Account {}] Withdrew {}, balance: {}",
                            self.account_id, amount, self.balance
                        );
                        let _ = reply_tx.send(Ok(()));
                    } else {
                        let _ = reply_tx.send(Err("Insufficient funds".to_string()));
                    }
                }
                AccountMessage::Balance(reply_tx) => {
                    let _ = reply_tx.send(self.balance);
                }
                AccountMessage::Stop => break,
            }
        }

        println!("[Account {}] Closed", self.account_id);
    }
}

#[derive(Clone)]
struct AccountHandle {
    sender: Sender<AccountMessage>,
}

impl AccountHandle {
    fn spawn(account_id: &str) -> (Self, JoinHandle<()>) {
        let (tx, rx) = mpsc::channel();
        let id = account_id.to_string();

        let handle = thread::spawn(move || {
            let mut actor = BankAccountActor::new(rx, &id);
            actor.run();
        });

        (Self { sender: tx }, handle)
    }

    fn deposit(&self, amount: u64) {
        let _ = self.sender.send(AccountMessage::Deposit(amount));
    }

    fn withdraw(&self, amount: u64) -> Result<(), String> {
        let (tx, rx) = mpsc::channel();
        let _ = self.sender.send(AccountMessage::Withdraw(amount, tx));
        rx.recv().unwrap_or(Err("Actor unavailable".to_string()))
    }

    fn balance(&self) -> u64 {
        let (tx, rx) = mpsc::channel();
        let _ = self.sender.send(AccountMessage::Balance(tx));
        rx.recv().unwrap_or(0)
    }

    fn stop(&self) {
        let _ = self.sender.send(AccountMessage::Stop);
    }
}

fn main() {
    println!("=== Counter Actor ===\n");

    let (counter, counter_join) = CounterHandle::spawn();

    counter.increment();
    counter.increment();
    counter.increment();
    counter.decrement();

    println!("Current value: {}", counter.get());

    counter.reset();
    println!("After reset: {}", counter.get());

    counter.stop();
    counter_join.join().unwrap();

    println!("\n=== Bank Account Actor ===\n");

    let (account, account_join) = AccountHandle::spawn("ACC-001");

    account.deposit(1000);
    account.deposit(500);

    println!("Balance: ${}", account.balance());

    match account.withdraw(300) {
        Ok(()) => println!("Withdrawal successful"),
        Err(e) => println!("Withdrawal failed: {}", e),
    }

    println!("Balance after withdrawal: ${}", account.balance());

    match account.withdraw(2000) {
        Ok(()) => println!("Withdrawal successful"),
        Err(e) => println!("Withdrawal failed: {}", e),
    }

    account.stop();
    account_join.join().unwrap();
}
