use std::{error::Error, io};

#[derive(Debug)]
struct Table {
    column_headers: Vec<String>,
    rows: Vec<Vec<String>>,
}


impl Table {
    fn new() -> Self {
        Table {
            column_headers: Vec::new(),
            rows: Vec::from(Vec::new())
        }
    }

    fn parse_table(self: &mut Self, tables: scraper::ElementRef) -> () {

        // get the column headers
        let head_selector = scraper::Selector::parse("thead>tr").unwrap();
        for head_row in tables.select(&head_selector) { 

            // get the column header cells
            let columns_selector = scraper::Selector::parse("th").unwrap();
            for column in head_row.select(&columns_selector) {
                self.column_headers.push(column.inner_html());
                println!("Inner HTML: {:?}", column.inner_html());
            }
        }

        // get the body rows
        let body_selector = scraper::Selector::parse("tbody>tr").unwrap();
        for row in tables.select(&body_selector) { 

            let mut acc: Vec<String> = Vec::new();

            //get the cells
            let cell_selector = scraper::Selector::parse("td").unwrap();
            for cell in row.select(&cell_selector) {

                let mut str = String::new();
                cell.text().for_each(|val| str.push_str(val));

                acc.push(str)
            }

            self.rows.push(acc);
        }

    }

    fn stdout_csv(self: &Self) -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::Writer::from_writer(io::stdout());

        wtr.write_record(&self.column_headers)?;

        for row in &self.rows {
            wtr.write_record(row)?;
        }

        wtr.flush()?;
        Ok(())
    }
}

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The url of the html to parse
    url: String 
}

fn main() {
    let args = Cli::parse();

    let body = reqwest::blocking::get(&args.url)
        .expect("Could not connect to URL...")
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&body); 

    let table_selector = scraper::Selector::parse("table").unwrap();

    for tables in document.select(&table_selector) {
        // initialize the struct with empty values
        let mut table = Table::new();

        // parse each table into the struct initialized above
        table.parse_table(tables);

        // call the method that will write the csv data to the stdout, which can then be piped into
        // another thing (DF, csv file, json, etc... via nushell)
        table.stdout_csv().unwrap();
    }
}
