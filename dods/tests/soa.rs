use dods::SoA;

#[derive(SoA, Debug, PartialEq, Clone)]
struct SensorReading {
    temperature: f32,
    pressure: f32,
    timestamp: u64,
}

#[test]
fn soa_push_swap_remove_pop_len() {
    let mut soa = SensorReadingSoA::new();

    let a = SensorReading {
        temperature: 10.5,
        pressure: 101.3,
        timestamp: 1,
    };
    let b = SensorReading {
        temperature: 11.0,
        pressure: 102.7,
        timestamp: 2,
    };

    soa.push(a.clone());
    soa.push(b.clone());
    assert_eq!(soa.len(), 2);

    let removed = soa.swap_remove(0);
    assert_eq!(soa.len(), 1);
    assert!(removed == a || removed == b);

    let remaining = soa.pop();
    assert_eq!(soa.len(), 0);
    assert!(remaining == Some(a) || remaining == Some(b));

    let empty = soa.pop();
    assert_eq!(empty, None);
}

#[test]
fn soa_get() {
    let mut soa = SensorReadingSoA::new();
    let a = SensorReading {
        temperature: 10.5,
        pressure: 101.3,
        timestamp: 1,
    };
    let b = SensorReading {
        temperature: 11.0,
        pressure: 102.7,
        timestamp: 2,
    };

    soa.push(a.clone());
    soa.push(b.clone());

    let got = soa.get(0).unwrap();
    assert_eq!(*got.temperature, a.temperature);
    assert_eq!(*got.pressure, a.pressure);
    assert_eq!(*got.timestamp, a.timestamp);

    assert!(soa.get(2).is_none());
}

#[test]
fn soa_iter() {
    let mut soa = SensorReadingSoA::new();
    let a = SensorReading {
        temperature: 10.5,
        pressure: 101.3,
        timestamp: 1,
    };
    let b = SensorReading {
        temperature: 11.0,
        pressure: 102.7,
        timestamp: 2,
    };

    soa.push(a.clone());
    soa.push(b.clone());

    let collected: Vec<SensorReading> = soa
        .iter()
        .map(|r| SensorReading {
            temperature: *r.temperature,
            pressure: *r.pressure,
            timestamp: *r.timestamp,
        })
        .collect();

    assert_eq!(collected, vec![a, b]);
}

#[test]
fn soa_iter_mut() {
    let mut soa = SensorReadingSoA::new();
    let a = SensorReading {
        temperature: 10.5,
        pressure: 101.3,
        timestamp: 1,
    };
    let b = SensorReading {
        temperature: 11.0,
        pressure: 102.7,
        timestamp: 2,
    };

    soa.push(a.clone());
    soa.push(b.clone());

    for mut r in soa.iter_mut() {
        *r.temperature += 1.0;
    }

    assert_eq!(*soa.get(0).unwrap().temperature, a.temperature + 1.0);
    assert_eq!(*soa.get(1).unwrap().temperature, b.temperature + 1.0);
}
