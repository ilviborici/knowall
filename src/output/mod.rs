/*!
 * For pretty printing the output to terminal.
 *
 * > This requires `cli` feature to be enabled. (enabled by default)
 *
 * # Example:
 *
 * Print results in a beautiful table!
 *
 * ```rust
 * use lemmeknow::{Identifier, PrintMode};
 * let identifier = Identifier::default();
 * let result = identifier.identify("UC11L3JDgDQMyH8iolKkVZ4w");
 * let printer = PrintMode::Normal;
 * printer.print(&result);
 * ```
*/

use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

use crate::Match;

/// Modes defining how the output shall be printed
/// > Requires `cli` feature
pub enum PrintMode {
    Normal,
    Verbose,
}

impl PrintMode {
    /// Print [`Vec<Match>`] in a tabular form.
    ///
    /// Use this if you want to print the possible identification in terminal
    /// with a pretty table.
    ///
    /// * [`PrintMode::Normal`] will print "Matched text", "Identified as" and "Description" columns.
    /// * [`PrintMode::Verbose`] will print "Rarity" and "Tags" along with other columns.
    ///
    /// # Arguments
    ///
    /// * result: `&Match` - Reference to [`Vec<Match>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use lemmeknow::{Identifier, PrintMode};
    /// let identifier = Identifier::default();
    /// let result = identifier.identify("UC11L3JDgDQMyH8iolKkVZ4w");
    /// let printer = PrintMode::Verbose;
    ///
    /// printer.print(&result);
    /// ```
    ///
    pub fn print(self, result: &[Match]) {
        pretty_print(result, self)
    }
}

fn pretty_print(result: &[Match], output_format: PrintMode) {
    let mut table = Table::new();
    let mut headers = vec![
        Cell::new("Matched text")
            .add_attribute(Attribute::Bold)
            .fg(Color::Magenta),
        Cell::new("Identified as")
            .add_attribute(Attribute::Bold)
            .fg(Color::Magenta),
        Cell::new("Description")
            .add_attribute(Attribute::Bold)
            .fg(Color::Magenta),
    ];

    if let PrintMode::Verbose = output_format {
        headers.extend([
            Cell::new("Rarity")
                .add_attribute(Attribute::Bold)
                .fg(Color::Magenta),
            Cell::new("Tags")
                .add_attribute(Attribute::Bold)
                .fg(Color::Magenta),
        ]);
    }

    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        // .set_table_width(80)
        .set_header(headers);

    if result.is_empty() {
        println!("\x1b[0;31mNo Possible Identifications :(\x1b[0m");
    } else {
        println!("\x1b[0;32mFound Possible Identifications :)\x1b[0m");

        result.iter().for_each(|item| {
            let mut description = String::from(item.data.description.unwrap_or_default());

            if let Some(url) = item.data.url {
                description.push_str(&format!(" URL: {url}{}\n", &item.text))
            }

            if let Some(exploit) = item.data.exploit {
                description.push_str(&format!("Exploit: {exploit}"))
            }

            if description.is_empty() {
                description.push_str("None")
            }

            let mut row = vec![
                Cell::new(&item.text),
                Cell::new(item.data.name),
                Cell::new(description),
            ];

            if let PrintMode::Verbose = output_format {
                row.extend([
                    Cell::new(item.data.rarity),
                    Cell::new(item.data.tags.join(", ")),
                ]);
            }

            table.add_row(row);
        });

        println!("{table}");
    }
}
