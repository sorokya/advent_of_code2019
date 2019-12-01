fn main() {
    let input: Vec<i32> = vec![
        123835, 66973, 63652, 99256, 56009, 58012, 130669, 109933, 52958, 131656, 144786, 50437,
        134194, 80230, 50326, 118204, 102780, 135520, 142248, 80341, 51071, 71346, 134081, 142321,
        136230, 55934, 79697, 90116, 107825, 133052, 130259, 99566, 83066, 90923, 58475, 134697,
        91830, 105838, 109003, 125258, 108679, 87310, 79813, 109814, 65616, 69275, 118405, 105178,
        93140, 79535, 138051, 55728, 71875, 121207, 52011, 81209, 129059, 135782, 62791, 72135,
        77765, 109498, 73862, 134825, 148898, 81633, 53277, 109858, 91672, 115105, 132871, 138334,
        135049, 73083, 79234, 129281, 86062, 88448, 99612, 52138, 149290, 120562, 118975, 92896,
        51162, 122410, 75479, 137800, 142149, 123518, 67806, 89937, 85963, 104764, 56710, 51314,
        67275, 61135, 77580, 74726,
    ];

    /*
        During the second Go / No Go poll, the Elf in charge of the Rocket Equation Double-Checker stops
         the launch sequence. Apparently, you forgot to include additional fuel for the fuel you just added.

    Fuel itself requires fuel just like a module - take its mass, divide by three, round down, and subtract 2.
     However, that fuel also requires fuel, and that fuel requires fuel, and so on. Any mass that would require
      negative fuel should instead be treated as if it requires zero fuel; the remaining mass, if any, is instead
       handled by wishing really hard, which has no mass and is outside the scope of this calculation.

    So, for each module mass, calculate its fuel and add it to the total.
    Then, treat the fuel amount you just calculated as the input mass and repeat the process,
    continuing until a fuel requirement is zero or negative.
        */
    let mut sum = 0;
    for i in input {
        sum += get_total_fuel_for_mass(i);
    }

    println!("Answer is {}", sum);
}

fn get_total_fuel_for_mass(mass: i32) -> i32 {
    let mut sum = 0;

    let mut temp = mass;
    while get_fuel_for_item(temp) > 0 {
        temp = get_fuel_for_item(temp);
        sum += temp;
    }

    sum
}

fn get_fuel_for_item(i: i32) -> i32 {
    let fuel = i / 3 - 2;
    if fuel <= 0 {
        return 0;
    }

    fuel
}
