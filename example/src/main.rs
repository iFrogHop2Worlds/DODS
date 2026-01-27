use dods::DodsSoA;

#[derive(DodsSoA, Debug, PartialEq, Clone)]
struct SensorReading {
    temperature: f32,
    pressure: f32,
    timestamp: u64,
}

fn main() {
    let mut station = SensorReadingSoA::new();

    for i in 0..1000 {
        station.push(SensorReading {
            temperature: 20.0 + (i as f32 * 0.1),
            pressure: 1211.0 + (i as f32 * 0.05),
            timestamp: 6969696969 + i,
        });
    }

    println!("Initialized station with {} sensors.", station.len());

    apply_heat_wave(&mut station, 5.5);

    let malfunctioned = station.swap_remove(42);

    println!("Removed malfunctioned sensor: {:?}", malfunctioned);
    println!("New length after O(1) swap_remove: {}", station.len());

    if let Some(reading) = station.get(10) {
        println!(
            "Sample Reading at index 10: temp={}, pressure={}, timestamp={}",
            *reading.temperature, *reading.pressure, *reading.timestamp
        );
    }

    let avg_temp = station.iter().map(|r| *r.temperature).sum::<f32>() / station.len() as f32;
    println!("Average temperature: {:.2}", avg_temp);
}

fn apply_heat_wave(station: &mut SensorReadingSoA, increase: f32) {
    for mut r in station.iter_mut() {
        *r.temperature += increase;
    }
}
