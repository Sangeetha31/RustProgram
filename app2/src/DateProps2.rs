use chrono::*;
use chrono_tz::Tz;

fn printtime(city: &str,timezone:Tz){
    let now=Utc::now().with_timezone(&timezone);
    println!("{:<15} {}",city,now.format("%d-%m-%Y %H:%M:%S %p"));
}
fn main(){
    printtime("India",chrono_tz::Asia::Kolkata);
    printtime("Australia - Sydney",chrono_tz::Australia::Sydney);
    printtime("USA - New York",chrono_tz::America::New_York);
    printtime("USA - Chicago",chrono_tz::America::Chicago);
    printtime("USA - Los Angeles",chrono_tz::America::Los_Angeles);
    printtime("Canada - Toronto",chrono_tz::America::Toronto);
    printtime("UK - London",chrono_tz::Europe::London);
    printtime("Dubai",chrono_tz::Asia::Dubai);

}