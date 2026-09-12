//! Compatibility transforms for Markdown syntax emitted by earlier editors.

/// Rewrites legacy `@cols=N:` table headings into regular Markdown table rows.
///
/// Legacy headings are only converted when the immediately following divider
/// has exactly `N` columns, avoiding changes to unrelated table content.
pub(crate) fn normalize_legacy_table_headers(markdown: &str) -> String {
    let line_ending = if markdown.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let has_trailing_line_ending = markdown.ends_with('\n');
    let lines = markdown
        .split_terminator('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line))
        .collect::<Vec<_>>();
    let mut normalized = Vec::with_capacity(lines.len());
    let mut changed = false;

    for (index, line) in lines.iter().enumerate() {
        let replacement = legacy_table_heading(line).and_then(|(columns, heading)| {
            (table_divider_columns(lines.get(index + 1).copied()) == Some(columns))
                .then(|| regular_table_heading(heading, columns))
        });
        if let Some(replacement) = replacement {
            normalized.push(replacement);
            changed = true;
        } else {
            normalized.push((*line).to_owned());
        }
    }

    if changed {
        let mut markdown = normalized.join(line_ending);
        if has_trailing_line_ending {
            markdown.push_str(line_ending);
        }
        markdown
    } else {
        markdown.to_owned()
    }
}

/// Extracts the declared column count and title from a legacy table heading.
fn legacy_table_heading(line: &str) -> Option<(usize, &str)> {
    let row = line.trim().strip_prefix('|')?.strip_suffix('|')?.trim();
    if row.contains('|') {
        return None;
    }

    let (columns, heading) = row.strip_prefix("@cols=")?.split_once(':')?;
    let columns = columns
        .parse::<usize>()
        .ok()
        .filter(|columns| *columns > 1)?;
    (!heading.trim().is_empty()).then_some((columns, heading.trim()))
}

/// Counts the columns in a standard Markdown table divider when every cell is valid.
fn table_divider_columns(line: Option<&str>) -> Option<usize> {
    let row = line?.trim().strip_prefix('|')?.strip_suffix('|')?;
    let cells = row.split('|').collect::<Vec<_>>();
    (!cells.is_empty() && cells.iter().all(|cell| is_table_divider_cell(cell)))
        .then_some(cells.len())
}

/// Checks whether one cell is a valid Markdown table divider segment.
fn is_table_divider_cell(cell: &str) -> bool {
    let dashes = cell.trim().trim_matches(':');
    dashes.len() >= 3 && dashes.bytes().all(|character| character == b'-')
}

/// Builds a regular table row whose first cell contains the legacy title.
fn regular_table_heading(heading: &str, columns: usize) -> String {
    format!("| {heading} |{}", " |".repeat(columns - 1))
}

#[cfg(test)]
mod tests {
    use super::normalize_legacy_table_headers;

    /// Converts a legacy spanning header when its divider declares the same width.
    #[test]
    fn normalizes_legacy_table_heading() {
        let markdown =
            "| @cols=2:**Befunde Erstuntersuchung** |\n| --- | --- |\n| **BCS:** | 2-3 |";

        assert_eq!(
            normalize_legacy_table_headers(markdown),
            "| **Befunde Erstuntersuchung** | |\n| --- | --- |\n| **BCS:** | 2-3 |"
        );
    }

    /// Leaves a legacy marker unchanged when its divider has a different width.
    #[test]
    fn preserves_unmatched_legacy_table_heading() {
        let markdown = "| @cols=2:**Heading** |\n| --- | --- | --- |";

        assert_eq!(normalize_legacy_table_headers(markdown), markdown);
    }
}
