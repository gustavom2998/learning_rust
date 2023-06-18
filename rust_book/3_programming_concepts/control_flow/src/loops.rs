fn count_evens(){
    // Using a loop to count number of even numbers between (0,num]
    println!("Using a loop to count number of even numbers between (0,num]");

    let num = 8;
    let mut counter = 0;

    println!("Count to {num} skipping odd numbers");
    loop{
        counter += 1;
        if (counter % 2 == 1) && (counter <= num) {
            continue;
        }
        else if counter <= num {
            println!("Current count: {counter}/{num}");
        }
        else {
            println!("Finished count");
            break;
        }
    }
}

fn sum_evens(){
    // Using a loop to return sum of even numbers between (0,num]
    println!("Using a loop to return sum of even numbers between (0,num]");

    let num = 8;
    let mut counter = 0;
    let mut sum = 0;

    let sum_res = loop{
        counter += 1;
        sum += if counter % 2 == 0 {counter} else {0};

        if counter > num {
            break sum;
        }
    };
    println!("Sumation to {num} skipping odd numbers: {sum_res}");
}

fn sum_even_factorials (){
    // Using nested loop to return sum factorials of even numbers between (0,num]
    println!("Using nested loop to return sum factorials of even numbers between (0,num]");

    let num = 4;
    let mut counter = 0;
    let mut sum = 0;

    'sum_facts : loop { 
        counter += 1;
        let mut fact_val = 1;
        let mut fact_counter = num;

        'fact : loop {
            fact_val *= fact_counter;
            fact_counter -= 1;

            if fact_counter <= 1 {
                break 'fact;
            }
        }

        sum += if counter % 2 == 0 {fact_val} else {0}; 

        if counter > num {
            break 'sum_facts;
        }
    }
    println!("Sumation of factorial(n={num}) skipping odd n's: {sum}");
}

fn countdown(){
    // Using a while loop to validate a condition
    println!("Using a while loop to validate a condition");
    let mut countdown = 5;

    while countdown > 0 {
        println!("{countdown}");
        countdown -= 1;
    }
    println!("Liftoff!");
}

fn array_iteration(){
    // Iterate an array with for
    println!("Iterate an array with for");
    let arr = [1,2,3,4];

    for item in arr {
        println!("{item}");
    }
}

fn countdown_for(){
    //  Using for loop to countdown
    println!("Using for loop to countdown");
    let countdown = 5;
    for item in (1..countdown + 1).rev() {
        println!("{item}");
    }
    println!("Liftoff!");
}

fn main(){
    count_evens();
    sum_evens();
    sum_even_factorials();
    countdown();
    array_iteration();
    countdown_for();
}
