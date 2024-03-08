use std::io;

fn main() {
    println!("Enter the size of the multiplication table:");
    let mut size_str = String::new();
    io::stdin().read_line(&mut size_str).expect("Failed to read line");
    let size: usize = size_str.trim().parse().expect("Please type a number!");

    print_multiplication_table(size);
}

fn print_multiplication_table(size: usize) {
    let cell_width = ((size * size).to_string().len() + 1).max(4);

    print!("┌{}┐\n", "─".repeat(cell_width * (size + 1) + size));

    print!("│{:width$}│", "", width = cell_width);
    for i in 1..=size {
        print!("{:width$}│", i, width = cell_width);
    }
    println!();

    print!("├{}┤\n", "─".repeat(cell_width * (size + 1) + size));

    for row in 1..=size {
        print!("│{:width$}│", row, width = cell_width);
        
        for col in 1..=size {
            print!("{:width$}│", row * col, width = cell_width);
        }
        println!();

        if row != size {
            print!("├{}┤\n", "─".repeat(cell_width * (size + 1) + size));
        }
    }

    print!("└{}┘\n", "─".repeat(cell_width * (size + 1) + size));
}
