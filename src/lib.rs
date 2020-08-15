use rand::Rng;

pub struct Roll {
    pub rolls: i32,
    pub sides: i32,
    pub modifier: &'static str,
    pub mod_number: i32,
}

pub fn parse_roll(mut args: std::env::Args) -> Result<Vec<Roll>, &'static str> {
    args.next();

    let uprolls:Vec<String> = args
                .filter(|roll| roll.contains("d") )
                .collect();

    let mut parsed_rolls: Vec<Roll> = Vec::new();

    for roll in uprolls{
        let split: Vec<&str> = roll.split("d").collect();
        let rolls: i32 = match split[0].parse() {
            Ok(rolls) => rolls,
            Err(_) => continue,
        };
        
        if rolls < 0 {continue};

        let split: &str = split[1];        
        
        if split.contains("+") {
            let split: Vec<&str> =  split.split("+").collect();
            let sides: i32 = match split[0].parse() {
                Ok(sides) => sides,
                Err(_) => continue,
            };
            let modifier = "+";
            let mod_number: i32 = match split[1].parse() {
                Ok(mod_number) => mod_number,
                Err(_) => continue,
            };

            if sides < 0 {continue};

            let proll = Roll {
                rolls,
                sides,
                modifier,
                mod_number,
            };

            parsed_rolls.push(proll);

        }else if split.contains("-") {
            let split: Vec<&str> =  split.split("-").collect();
            let sides: i32 = match split[0].parse() {
                Ok(sides) => sides,
                Err(_) => continue,
            };
            let modifier = "-";
            let mod_number: i32 = match split[1].parse() {
                Ok(mod_number) => mod_number,
                Err(_) => continue,
            };

            if sides < 0 {continue};

            let proll = Roll {
                rolls,
                sides,
                modifier,
                mod_number,
            };

            parsed_rolls.push(proll);

        }else {
            let modifier = "NaN";
            let mod_number = 0;

            let sides: i32 = match split.parse() {
                Ok(sides) => sides,
                Err(_) => continue,
            };

            if sides < 0 {continue};

            let proll = Roll {
                rolls,
                sides,
                modifier,
                mod_number,
            };
            parsed_rolls.push(proll);
        }
    }

    if !parsed_rolls.is_empty() {
        return Ok(parsed_rolls);
    }else{
        return Err("No valid rolls given, try to run the command with valid rolls \neg: diceroll 1d20 1d20+2 1d20-10");
    }
}

pub fn throw_rolls(rolls: Vec<Roll>) {
    for roll in rolls {

        if roll.modifier == "NaN"{
            println!("Throwing {}d{}:", roll.rolls, roll.sides);
        }else{
            println!("Throwing {}d{} {}{}:", roll.rolls, roll.sides, roll.modifier, roll.mod_number);
        }
        
        let mut sum = 0;
        if roll.sides == 10 || roll.sides == 100 {
            for _throw in 0..roll.rolls {
                let rroll = rand::thread_rng().gen_range(0, roll.sides + 1);
                print!("{} ", rroll);

                sum += rroll;
            }
        }else{
            for _throw in 0..roll.rolls {
                let rroll = rand::thread_rng().gen_range(1, roll.sides + 1);
                print!("{} ", rroll);

                sum += rroll;
            }
        }

        if roll.modifier == "+" {
            print!("+{}\n", roll.mod_number);
            sum = sum + roll.mod_number;
        }else if roll.modifier == "-" {
            print!("-{}\n", roll.mod_number);
            sum = sum - roll.mod_number;
        }else{
            print!("\n");
        }

        println!("Sum is: {}\n", sum);
    }
}