use std::cmp::Ordering;

//this implements the trait for enum or struct that provide way to format the types's data
//in human readable way
#[derive(Debug)]
struct Item {
    profit: f64,
    weight: f64,
}

impl Item {
    fn new(profit: f64, weight: f64) -> Self {
        Item { profit, weight }
    }

    fn ratio(&self) -> f64 {
        self.profit / self.weight
    }
}

fn cmp(a: &Item, b: &Item) -> Ordering {
    b.ratio().partial_cmp(&a.ratio()).unwrap_or(Ordering::Equal)
}

fn fractional_knapsack(max_weight: f64, items: &mut [Item]) -> f64 {
    items.sort_by(cmp);

    let mut remaining_weight = max_weight;
    let mut final_value = 0.0;

    for item in items {
        if item.weight <= remaining_weight {
            remaining_weight -= item.weight;
            final_value += item.profit;
        } else {
            final_value += item.profit * (remaining_weight / item.weight);
            break;
        }
    }

    final_value
    // let temp: f64 = 1.0;
    // temp
}

fn main() {
    let max_weight: f64 = 50.0;
    let mut items: Vec<Item> = Vec::new();
    items.push(Item::new(60.0, 13.0));
    items.push(Item::new(100.0, 21.0));
    items.push(Item::new(120.0, 33.0));

    let max_profit = fractional_knapsack(max_weight, &mut items);
    println!("Maximum Profit : {:.2}",max_profit);
}
