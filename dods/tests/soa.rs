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

#[test]
fn soa_basic_accessors_and_slices() {
    let mut soa = SensorReadingSoA::with_capacity(4);
    assert!(soa.is_empty());
    assert!(soa.capacity() >= 4);

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
    let c = SensorReading {
        temperature: 12.0,
        pressure: 103.9,
        timestamp: 3,
    };

    soa.push(a.clone());
    soa.push(b.clone());
    soa.push(c.clone());

    assert!(!soa.is_empty());
    assert_eq!(*soa.first().unwrap().timestamp, a.timestamp);
    assert_eq!(*soa.last().unwrap().timestamp, c.timestamp);
    assert_eq!(*soa.index(1).pressure, b.pressure);

    let slice = soa.as_slice();
    assert_eq!(slice.temperature.len(), 3);
    assert_eq!(slice.timestamp[2], c.timestamp);

    let mid = soa.slice(1..=1);
    assert_eq!(mid.pressure, [b.pressure]);

    let head = soa.slice(..2);
    assert_eq!(head.temperature, [a.temperature, b.temperature]);

    let tail = soa.slice(1..);
    assert_eq!(tail.timestamp, [b.timestamp, c.timestamp]);

    let mut_slice = soa.as_mut_slice();
    mut_slice.temperature[0] += 1.0;
    assert_eq!(*soa.first().unwrap().temperature, a.temperature + 1.0);

    let mid_mut = soa.slice_mut(1..3);
    mid_mut.pressure[1] += 1.0;
    assert_eq!(*soa.last().unwrap().pressure, c.pressure + 1.0);
}

#[test]
fn soa_mut_accessors_and_pointers() {
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

    assert_eq!(*soa.get_mut(0).unwrap().temperature, a.temperature);
    *soa.index_mut(1).pressure += 1.0;
    assert_eq!(*soa.last().unwrap().pressure, b.pressure + 1.0);

    *soa.first_mut().unwrap().timestamp += 10;
    assert_eq!(*soa.get(0).unwrap().timestamp, a.timestamp + 10);

    *soa.last_mut().unwrap().temperature += 2.0;
    assert_eq!(*soa.get(1).unwrap().temperature, b.temperature + 2.0);

    let ptrs = soa.as_ptr();
    unsafe {
        assert_eq!(*ptrs.timestamp, a.timestamp + 10);
    }

    let mut_ptrs = soa.as_mut_ptr();
    unsafe {
        *mut_ptrs.pressure.add(0) += 2.0;
    }
    assert_eq!(*soa.get(0).unwrap().pressure, a.pressure + 2.0);
}

#[test]
fn soa_insert_replace_remove_and_capacity_ops() {
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
    let c = SensorReading {
        temperature: 12.0,
        pressure: 103.9,
        timestamp: 3,
    };

    soa.reserve(4);
    assert!(soa.capacity() >= 4);
    soa.reserve_exact(1);

    soa.push(a.clone());
    soa.insert(1, b.clone());
    assert_eq!(soa.len(), 2);
    assert_eq!(*soa.get(1).unwrap().timestamp, b.timestamp);

    let replaced = soa.replace(1, c.clone());
    assert_eq!(replaced, b);
    assert_eq!(*soa.get(1).unwrap().timestamp, c.timestamp);

    let removed = soa.remove(0);
    assert_eq!(removed, a);
    assert_eq!(soa.len(), 1);

    soa.truncate(0);
    assert_eq!(soa.len(), 0);

    soa.push(c);
    soa.shrink_to_fit();
    soa.clear();
    assert!(soa.is_empty());
}

#[test]
fn soa_append_split_off_and_sorting() {
    let mut left = SensorReadingSoA::new();
    let mut right = SensorReadingSoA::new();

    left.push(SensorReading {
        temperature: 30.0,
        pressure: 110.0,
        timestamp: 3,
    });
    left.push(SensorReading {
        temperature: 10.0,
        pressure: 100.0,
        timestamp: 1,
    });
    right.push(SensorReading {
        temperature: 20.0,
        pressure: 105.0,
        timestamp: 2,
    });

    left.append(&mut right);
    assert_eq!(left.len(), 3);
    assert!(right.is_empty());

    let split = left.split_off(1);
    assert_eq!(left.len(), 1);
    assert_eq!(split.len(), 2);

    let mut sortable = SensorReadingSoA::new();
    sortable.push(SensorReading {
        temperature: 30.0,
        pressure: 110.0,
        timestamp: 3,
    });
    sortable.push(SensorReading {
        temperature: 10.0,
        pressure: 100.0,
        timestamp: 1,
    });
    sortable.push(SensorReading {
        temperature: 20.0,
        pressure: 105.0,
        timestamp: 2,
    });

    sortable.sort_by(|a, b| a.timestamp.cmp(b.timestamp));
    assert_eq!(*sortable.get(0).unwrap().timestamp, 1);
    assert_eq!(*sortable.get(2).unwrap().timestamp, 3);

    sortable.sort_by_key(|a| *a.temperature as i32);
    assert_eq!(*sortable.get(0).unwrap().temperature, 10.0);
    assert_eq!(*sortable.get(2).unwrap().temperature, 30.0);

    let mut manual = SensorReadingSoA::new();
    manual.push(SensorReading {
        temperature: 10.0,
        pressure: 100.0,
        timestamp: 1,
    });
    manual.push(SensorReading {
        temperature: 20.0,
        pressure: 105.0,
        timestamp: 2,
    });
    manual.push(SensorReading {
        temperature: 30.0,
        pressure: 110.0,
        timestamp: 3,
    });
    manual.apply_index(&[2, 0, 1]);
    assert_eq!(*manual.get(0).unwrap().timestamp, 3);
    assert_eq!(*manual.get(1).unwrap().timestamp, 1);
    assert_eq!(*manual.get(2).unwrap().timestamp, 2);
}
