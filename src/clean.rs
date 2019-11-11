use ical::parser::ical::component::{IcalCalendar, IcalEvent};
use icalendar::{Calendar, Component, Event, Property};
use std::collections::HashMap;

pub fn clean(input: Vec<IcalCalendar>) -> Calendar {
    let mut output = Calendar::new();

    for cal in input {
        clean_cal(cal, &mut output);
    }

    output
}

fn clean_cal(cal: IcalCalendar, out: &mut Calendar) {
    for event in cal.events {
        let mut props = convert_event(event);

        fix_prop(&mut props);

        let mut event = Event::new();
        for (_key, prop) in props {
            event.append_property(Property::from(prop));
        }
        out.push(event);
    }
}

fn fix_prop(map: &mut HashMap<String, Prop>) {
    let sum = map.get("SUMMARY")
        .map(|prop| prop.value.as_str())
        .and_then(crate::summary::clean_summary);
    if let Some(sum) = sum {
        map.get_mut("SUMMARY").unwrap().value = sum.to_string();
    }
}


struct Prop {
    key: String,
    value: String,
    params: Vec<(String, String)>,
}
impl From<Prop> for Property {
    fn from(p: Prop) -> Property {
        let mut prop = Property::new(&p.key, &p.value);
        for (key, val) in p.params {
            prop.add_parameter(&key, &val);
        }
        prop
    }
}
fn convert_event(event: IcalEvent) -> HashMap<String, Prop> {
    let mut props = HashMap::new();
    for prop in event.properties {
        let mut res = Prop {
            key: prop.name.clone(),
            value: prop.value.unwrap_or(String::new()),
            params: Vec::new(),
        };
        for (key, vals) in prop.params.into_iter().flatten() {
            for val in vals {
                res.params.push((key.clone(), val));
            }
        }
        props.insert(prop.name, res);
    }
    props
}
