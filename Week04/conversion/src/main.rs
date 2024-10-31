fn main() {
    // Сега вече може да си сварим яйцата

    // Използвайки from
    let e = energy_to_heat_water(
        Kelvin::from(room_temperature()), 
        Kelvin::from(Celsius(100.0)),
        Kg(1.0)
    );

    // Използвайки into
    let e = energy_to_heat_water(
        room_temperature().into(), 
        Celsius(100.0).into(), 
        Kg(1.0)
    );

    let e = energy_to_heat_water2(
        room_temperature(), 
        Celsius(100.0), 
        Kg(1.0)
    );
}

struct Celsius(f64);
struct Farenheit(f64);
struct Kelvin(f64);
struct Kg(f64);
struct Joule(f64);

fn room_temperature() -> Farenheit {
    Farenheit(68.0)
}

fn energy_to_heat_water(from: Kelvin, to: Kelvin, mass: Kg) -> Joule {
    todo!()
}

impl From<Celsius> for Kelvin {
    fn from(t: Celsius) -> Kelvin {
        Kelvin(t.0 + 273.15)
    }
}

impl From<Farenheit> for Celsius {
    fn from(t: Farenheit) -> Celsius {
        Celsius((t.0 - 32_f64) / 1.8)
    }
}

impl From<Farenheit> for Kelvin {
    fn from(t: Farenheit) -> Kelvin {
        Kelvin::from(Celsius::from(t))
    }
}

fn energy_to_heat_water2<T1, T2>(from: T1, to: T2, mass: Kg)
where 
    T1: Into<Kelvin>,
    T2: Into<Kelvin>
{
    let from = from.into();
    let to   = to.into(); 

    //whaterver
}