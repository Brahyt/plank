use clap::Parser;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    db_url: String
}

//#[derive(Queryable, Selectable)]
//#[diesel(table_name = git_urls)]
//pub struct Download {
//    id: i32,
//    url: String,
//    name: String,
//}

pub fn establish_connection(db_url: &str) -> PgConnection {

    let database_url = db_url;

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let args = Args::parse();

    let connection = establish_connection(&args.db_url);

    println!("Hello, world!, {:#?}", connection);
 }
