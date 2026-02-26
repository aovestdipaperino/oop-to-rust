//! Chapter 11: Behavioral Patterns - Strategy Pattern

trait PaymentStrategy {
    fn pay(&self, amount: f64) -> Result<String, String>;
    fn name(&self) -> &str;
}

struct CreditCardPayment {
    card_number: String,
}

impl CreditCardPayment {
    fn new(card_number: &str) -> Self {
        Self {
            card_number: card_number.to_string(),
        }
    }
}

impl PaymentStrategy for CreditCardPayment {
    fn pay(&self, amount: f64) -> Result<String, String> {
        let masked = format!(
            "****-****-****-{}",
            &self.card_number[self.card_number.len() - 4..]
        );
        Ok(format!("Paid ${:.2} with credit card {}", amount, masked))
    }
    fn name(&self) -> &str {
        "Credit Card"
    }
}

struct PayPalPayment {
    email: String,
}

impl PayPalPayment {
    fn new(email: &str) -> Self {
        Self {
            email: email.to_string(),
        }
    }
}

impl PaymentStrategy for PayPalPayment {
    fn pay(&self, amount: f64) -> Result<String, String> {
        Ok(format!("Paid ${:.2} via PayPal ({})", amount, self.email))
    }
    fn name(&self) -> &str {
        "PayPal"
    }
}

struct ShoppingCart {
    items: Vec<(String, f64)>,
}

impl ShoppingCart {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add_item(&mut self, name: &str, price: f64) {
        self.items.push((name.to_string(), price));
    }

    fn total(&self) -> f64 {
        self.items.iter().map(|(_, price)| price).sum()
    }

    fn checkout(&self, strategy: &dyn PaymentStrategy) -> Result<String, String> {
        let total = self.total();
        if total <= 0.0 {
            return Err("Cart is empty".to_string());
        }
        strategy.pay(total)
    }
}

// Closure-based strategy
struct PriceCalculator {
    base_price: f64,
}

impl PriceCalculator {
    fn new(base_price: f64) -> Self {
        Self { base_price }
    }

    fn calculate<F>(&self, discount_strategy: F) -> f64
    where
        F: Fn(f64) -> f64,
    {
        discount_strategy(self.base_price)
    }
}

fn no_discount(price: f64) -> f64 {
    price
}

fn percentage_discount(percent: f64) -> impl Fn(f64) -> f64 {
    move |price| price * (1.0 - percent / 100.0)
}

fn main() {
    println!("=== Payment Strategies ===\n");

    let mut cart = ShoppingCart::new();
    cart.add_item("Rust Book", 49.99);
    cart.add_item("Keyboard", 149.99);

    println!("Cart total: ${:.2}\n", cart.total());

    let strategies: Vec<Box<dyn PaymentStrategy>> = vec![
        Box::new(CreditCardPayment::new("4111111111111234")),
        Box::new(PayPalPayment::new("user@example.com")),
    ];

    for strategy in &strategies {
        println!("Paying with {}:", strategy.name());
        match cart.checkout(strategy.as_ref()) {
            Ok(msg) => println!("  {}", msg),
            Err(e) => println!("  Error: {}", e),
        }
    }

    println!("\n=== Discount Strategies (Closures) ===\n");

    let calc = PriceCalculator::new(100.0);

    println!("Base price: ${:.2}", calc.base_price);
    println!("No discount: ${:.2}", calc.calculate(no_discount));
    println!("10% off: ${:.2}", calc.calculate(percentage_discount(10.0)));
    println!("25% off: ${:.2}", calc.calculate(percentage_discount(25.0)));
}
