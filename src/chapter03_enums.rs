//! Chapter 3: Structs, Enums, and the Death of Null
//!
//! This example demonstrates modeling a domain with structs and enums,
//! showing how Rust's type system makes invalid states unrepresentable.

use std::time::SystemTime;

// Tuple structs for type-safe IDs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct OrderId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CustomerId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ProductId(u64);

// Order item with product details
#[derive(Debug, Clone)]
struct OrderItem {
    product_id: ProductId,
    quantity: u32,
    unit_price: u64, // Price in cents
}

impl OrderItem {
    fn new(product_id: ProductId, quantity: u32, unit_price: u64) -> Self {
        Self {
            product_id,
            quantity,
            unit_price,
        }
    }

    fn total(&self) -> u64 {
        self.quantity as u64 * self.unit_price
    }
}

// Shipping information (only relevant when shipped)
#[derive(Debug, Clone)]
struct ShippingInfo {
    carrier: String,
    tracking_number: String,
    shipped_at: SystemTime,
}

// Delivery information (only relevant when delivered)
#[derive(Debug, Clone)]
struct DeliveryInfo {
    delivered_at: SystemTime,
    signature: Option<String>,
}

// Order status as an enum with associated data
#[derive(Debug, Clone)]
enum OrderStatus {
    Pending,
    Shipped(ShippingInfo),
    Delivered {
        shipping: ShippingInfo,
        delivery: DeliveryInfo,
    },
    Cancelled(String), // Reason for cancellation
}

// The order struct
#[derive(Debug)]
struct Order {
    id: OrderId,
    customer_id: CustomerId,
    items: Vec<OrderItem>,
    status: OrderStatus,
}

impl Order {
    fn new(id: OrderId, customer_id: CustomerId) -> Self {
        Self {
            id,
            customer_id,
            items: Vec::new(),
            status: OrderStatus::Pending,
        }
    }

    fn add_item(&mut self, item: OrderItem) {
        self.items.push(item);
    }

    fn total(&self) -> u64 {
        self.items.iter().map(|item| item.total()).sum()
    }

    fn ship(&mut self, carrier: String, tracking_number: String) -> Result<(), &'static str> {
        match &self.status {
            OrderStatus::Pending => {
                self.status = OrderStatus::Shipped(ShippingInfo {
                    carrier,
                    tracking_number,
                    shipped_at: SystemTime::now(),
                });
                Ok(())
            }
            _ => Err("Can only ship pending orders"),
        }
    }

    fn deliver(&mut self, signature: Option<String>) -> Result<(), &'static str> {
        match &self.status {
            OrderStatus::Shipped(shipping) => {
                self.status = OrderStatus::Delivered {
                    shipping: shipping.clone(),
                    delivery: DeliveryInfo {
                        delivered_at: SystemTime::now(),
                        signature,
                    },
                };
                Ok(())
            }
            _ => Err("Can only deliver shipped orders"),
        }
    }

    fn cancel(&mut self, reason: String) -> Result<(), &'static str> {
        match &self.status {
            OrderStatus::Pending => {
                self.status = OrderStatus::Cancelled(reason);
                Ok(())
            }
            OrderStatus::Shipped(_) => Err("Cannot cancel shipped orders"),
            OrderStatus::Delivered { .. } => Err("Cannot cancel delivered orders"),
            OrderStatus::Cancelled(_) => Err("Order already cancelled"),
        }
    }

    // Returns tracking number only if order has been shipped
    fn tracking_number(&self) -> Option<&str> {
        match &self.status {
            OrderStatus::Shipped(info) => Some(&info.tracking_number),
            OrderStatus::Delivered { shipping, .. } => Some(&shipping.tracking_number),
            _ => None,
        }
    }

    fn status_description(&self) -> String {
        match &self.status {
            OrderStatus::Pending => "Pending".to_string(),
            OrderStatus::Shipped(info) => {
                format!("Shipped via {} ({})", info.carrier, info.tracking_number)
            }
            OrderStatus::Delivered { delivery, .. } => {
                let sig = delivery
                    .signature
                    .as_ref()
                    .map(|s| format!(" (signed by {})", s))
                    .unwrap_or_default();
                format!("Delivered{}", sig)
            }
            OrderStatus::Cancelled(reason) => format!("Cancelled: {}", reason),
        }
    }
}

fn main() {
    // Create a new order
    let mut order = Order::new(OrderId(1001), CustomerId(42));

    // Add items
    order.add_item(OrderItem::new(ProductId(101), 2, 2999)); // 2x $29.99
    order.add_item(OrderItem::new(ProductId(102), 1, 4999)); // 1x $49.99

    println!("Order {:?}", order.id);
    println!("Customer: {:?}", order.customer_id);
    println!("Total: ${:.2}", order.total() as f64 / 100.0);
    println!("Status: {}", order.status_description());
    println!("Tracking: {:?}", order.tracking_number());

    // Ship the order
    println!("\n--- Shipping order ---");
    order
        .ship("FedEx".to_string(), "FX123456789".to_string())
        .expect("Should ship successfully");
    println!("Status: {}", order.status_description());
    println!("Tracking: {:?}", order.tracking_number());

    // Try to cancel (should fail)
    println!("\n--- Attempting to cancel shipped order ---");
    match order.cancel("Changed my mind".to_string()) {
        Ok(()) => println!("Cancelled"),
        Err(e) => println!("Cannot cancel: {}", e),
    }

    // Deliver the order
    println!("\n--- Delivering order ---");
    order
        .deliver(Some("John Doe".to_string()))
        .expect("Should deliver successfully");
    println!("Status: {}", order.status_description());

    // Demonstrate a cancelled order
    println!("\n--- Creating and cancelling another order ---");
    let mut order2 = Order::new(OrderId(1002), CustomerId(42));
    order2.add_item(OrderItem::new(ProductId(103), 1, 9999));
    order2
        .cancel("Out of stock".to_string())
        .expect("Should cancel successfully");
    println!("Order 1002 status: {}", order2.status_description());
}
