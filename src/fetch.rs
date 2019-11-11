use std::error::Error;
use std::io::Cursor;

use ical::parser::ical::component::IcalCalendar;
use ical::parser::ical::IcalParser;

use tokio::runtime::current_thread::Runtime;
use reqwest::Client;

pub fn fetch(username: &str) -> Result<Vec<IcalCalendar>, Box<dyn Error>> {
    let url = format!(
        "https://personligtskema.ku.dk/ical.asp?objectclass=student&id={}",
        username,
    );
    let mut runtime = Runtime::new()?;
    let client = Client::new();
    let request = client.get(url.as_str());
    let response = runtime.block_on(request.send())?;
    if response.status() != 200 {
        return Err(From::from(format!("Response code {}", response.status())));
    }
    let response = runtime.block_on(response.bytes())?;

    let reader = Cursor::new(response);
    let parser = IcalParser::new(reader);

    let calendars = match parser.collect::<Result<Vec<_>, _>>() {
        Ok(ok) => ok,
        Err(err) => return Err(From::from(format!("{}", err))),
    };

    drop(client);
    runtime.run()?;

    Ok(calendars)
}
