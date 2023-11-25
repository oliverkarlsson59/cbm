use clap::{Parser, Subcommand, Args};
use rusqlite::{Connection, Result};


#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Insert(Insert),
    List,
    Open(Open)
}

#[derive(Args)]
struct Insert {
    name: String,
    url: String
}

#[derive(Args)]
struct Open {
    id: u32
}


#[derive(Debug)]
struct Bookmark {
    id: u32,
    name: String,
    url: String
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut conn = Connection::open("/home/oliver/.dbs/bm.db")?;

    match &cli.command {
        Some(Commands::Insert(args)) => {
            insert_bookmark(&mut conn, &args.name, &args.url)?;
        }
        Some(Commands::List) => {
            print_bookmarks(&mut conn)?;
        }
        Some(Commands::Open(args)) => {
            open_bookmark(&mut conn, &args.id)?;
        }
        _ => {

        }
        
    }

    Ok(())
}

fn insert_bookmark(conn: &mut Connection, name: &String, url: &String) -> Result<(), rusqlite::Error> {
    let tx = conn.transaction()?;
    let _ = tx.execute("insert into store (name, url) values (?1, ?2)", &[&name, &url])?;
    tx.commit()
}

fn print_bookmarks(conn: &mut Connection) -> Result<()> {
    let mut query = conn.prepare("select ROWID, name, url from store")?;
    let bm = query.query_map([], |row| {
        Ok(Bookmark {
            id: row.get(0)?,
            name: row.get(1)?,
            url: row.get(2)?

        })
    })?;

    for x in bm {
        println!("{:?}", x.unwrap());
    }

    Ok(())
}

fn open_bookmark(conn: &mut Connection, id: &u32) -> Result<()> {
            let bm = get_bookmark_by_id(conn, id);
            match bm {
                Ok(val) => {
                    let _ = open::with(val.url, "firefox");
                },
                Err(err) => println!("{}", err),
            }

    Ok(())
}

fn get_bookmark_by_id(conn: &mut Connection, id: &u32) -> Result<Bookmark, rusqlite::Error> {
    let mut query = conn.prepare("select ROWID, name, url from store where ROWID=:id;")?;
    let bm = query.query_row(&[&id.to_string()], |row| {
        Ok(Bookmark {
            id: row.get(0)?,
            name: row.get(1)?,
            url: row.get(2)?
        })
    })?;


    Ok(bm)

}
