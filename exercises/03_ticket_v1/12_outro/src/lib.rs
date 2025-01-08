// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.


pub struct Order {
    product_name: String,
    unit_price: u32,
    quantity: u32
};


impl Order {

    fn validate_product_name(product_name: &String) {
        if product_name.is_empty() {
            panic!("product_name cannot be empty");
        }
        if product_name.len() > 300 {
            panic!("product_name cannot be longer than 50 bytes");
        }

    }
    
    fn validate_quantity(quantity: &u32) {
        if quantity < 1 {
            panic!("quantity must be strictly geater than 0");
        }
    }

    fn validate_unit_price(unit_price: &u32) {
        if unit_price < 1 {
            panic!("unit_price must be strictly geater than 0");
        }

    }

    pub fn new(product_name: String, unit_price: u32, quantity: u32) -> Order {
        Order::validate_product_name(&product_name);
        Order::validate_quantity(&quantity);
        Order::validate_unit_price(&unit_price);

        Order{product_name, quantity, unit_price}
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }


    pub fn set_title(&mut self, new_title: String) {
        Ticket::validate_title(&new_title);
        self.title = new_title;

    }

    pub fn set_description(&mut self, new_description: String) {
        Ticket::validate_description(&new_description);
        self.description = new_description;

    }
    pub fn set_status(&mut self, new_status: String) {
        Ticket::validate_status(&new_status);
        self.status = new_status;

    }

}