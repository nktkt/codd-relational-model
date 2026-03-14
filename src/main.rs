// =============================================================================
// Codd's Relational Model — Rust Implementation
//
// Based on E.F. Codd, "A Relational Model of Data for Large Shared Data Banks"
// Communications of the ACM, Volume 13, Number 6, June 1970, pp. 377–387
//
// Implements: Relation (set of tuples), Projection, Permutation,
//             Natural Join, Composition, Restriction
// =============================================================================

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt;

// =============================================================================
// Core Types
// =============================================================================

/// An atomic value in a tuple — covers all data in the paper.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Value {
    Int(i64),
    Str(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(n) => write!(f, "{n}"),
            Self::Str(s) => write!(f, "{s}"),
        }
    }
}

/// A tuple is an ordered sequence of values.
type Tuple = Vec<Value>;

/// A relation: a named set of n-tuples with named columns.
/// As Codd defines: all rows are distinct, column ordering is significant.
struct Relation {
    name: String,
    columns: Vec<String>,
    tuples: BTreeSet<Tuple>,
}

// Convenience constructors for Value
const fn int(n: i64) -> Value {
    Value::Int(n)
}

fn val(s: &str) -> Value {
    Value::Str(s.to_string())
}

/// Build a Relation from column names and row data.
///
/// # Panics
/// Panics if column names are not unique or if any row's arity
/// does not match the number of columns.
fn relation(name: &str, columns: &[&str], rows: Vec<Tuple>) -> Relation {
    let degree = columns.len();

    // Enforce unique column names — required for correct natural join behavior
    let unique: HashSet<&&str> = columns.iter().collect();
    assert_eq!(
        unique.len(),
        degree,
        "Duplicate column names in relation '{name}'"
    );

    let mut tuples = BTreeSet::new();
    for row in rows {
        assert_eq!(row.len(), degree, "Row arity mismatch in '{name}'");
        tuples.insert(row);
    }
    Relation {
        name: name.to_string(),
        columns: columns.iter().map(ToString::to_string).collect(),
        tuples,
    }
}

// =============================================================================
// Display
// =============================================================================

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ncols = self.columns.len();

        // Guard: empty-column relation
        if ncols == 0 {
            return write!(f, "{} (0 columns, {} tuples)", self.name, self.tuples.len());
        }

        // Compute column widths
        let mut widths: Vec<usize> = self.columns.iter().map(String::len).collect();
        for row in &self.tuples {
            for (i, v) in row.iter().enumerate() {
                let w = format!("{v}").len();
                if w > widths[i] {
                    widths[i] = w;
                }
            }
        }

        // Relation name
        writeln!(f, "{} ({} tuples):", self.name, self.tuples.len())?;

        // Top border
        write!(f, "┌")?;
        for (i, w) in widths.iter().enumerate() {
            write!(f, "─{}─", "─".repeat(*w))?;
            if i + 1 < ncols {
                write!(f, "┬")?;
            }
        }
        writeln!(f, "┐")?;

        // Header
        write!(f, "│")?;
        for (i, col) in self.columns.iter().enumerate() {
            write!(f, " {:<width$} │", col, width = widths[i])?;
        }
        writeln!(f)?;

        // Separator
        write!(f, "├")?;
        for (i, w) in widths.iter().enumerate() {
            write!(f, "─{}─", "─".repeat(*w))?;
            if i + 1 < ncols {
                write!(f, "┼")?;
            }
        }
        writeln!(f, "┤")?;

        // Rows
        for row in &self.tuples {
            write!(f, "│")?;
            for (i, v) in row.iter().enumerate() {
                write!(f, " {:<width$} │", format!("{v}"), width = widths[i])?;
            }
            writeln!(f)?;
        }

        // Bottom border
        write!(f, "└")?;
        for (i, w) in widths.iter().enumerate() {
            write!(f, "─{}─", "─".repeat(*w))?;
            if i + 1 < ncols {
                write!(f, "┴")?;
            }
        }
        write!(f, "┘")?;

        Ok(())
    }
}

// =============================================================================
// Relational Operations (Section 2.1)
// =============================================================================

/// 2.1.2 Projection — Select columns by index (in given order), remove duplicates.
///
/// # Panics
/// Panics if any index is out of bounds.
fn projection(r: &Relation, indices: &[usize], name: &str) -> Relation {
    let degree = r.columns.len();
    for &i in indices {
        assert!(i < degree, "Projection index {i} out of bounds (degree {degree})");
    }

    let columns: Vec<String> = indices.iter().map(|&i| r.columns[i].clone()).collect();
    let mut tuples = BTreeSet::new();
    for row in &r.tuples {
        let projected: Tuple = indices.iter().map(|&i| row[i].clone()).collect();
        tuples.insert(projected);
    }
    Relation {
        name: name.to_string(),
        columns,
        tuples,
    }
}

/// 2.1.1 Permutation — Reorder all columns. Must be a permutation of 0..n.
///
/// # Panics
/// Panics if `new_order` is not a valid permutation.
fn permutation(r: &Relation, new_order: &[usize], name: &str) -> Relation {
    let n = r.columns.len();
    assert_eq!(new_order.len(), n, "Permutation must cover all columns");
    let mut sorted = new_order.to_vec();
    sorted.sort_unstable();
    assert_eq!(sorted, (0..n).collect::<Vec<_>>(), "Not a valid permutation");
    projection(r, new_order, name)
}

/// 2.1.3 Natural Join — Join two relations on all commonly-named columns.
///
/// Each common column name is matched exactly once (first occurrence in S).
/// If no columns are in common, the result is the Cartesian product.
fn natural_join(r: &Relation, s: &Relation, name: &str) -> Relation {
    // Find common columns — match each R column to at most one S column (1:1)
    let mut r_common_indices = Vec::new();
    let mut s_common_indices = Vec::new();
    let mut matched_s: HashSet<usize> = HashSet::new();

    for (ri, rc) in r.columns.iter().enumerate() {
        for (si, sc) in s.columns.iter().enumerate() {
            if rc == sc && !matched_s.contains(&si) {
                r_common_indices.push(ri);
                s_common_indices.push(si);
                matched_s.insert(si);
                break; // move to next R column
            }
        }
    }

    // Result columns: all of R, then non-common columns of S
    let s_extra_indices: Vec<usize> = (0..s.columns.len())
        .filter(|i| !s_common_indices.contains(i))
        .collect();

    let mut columns: Vec<String> = r.columns.clone();
    for &i in &s_extra_indices {
        columns.push(s.columns[i].clone());
    }

    // Build index on R's common columns for efficient join
    let mut r_index: BTreeMap<Tuple, Vec<&Tuple>> = BTreeMap::new();
    for row in &r.tuples {
        let key: Tuple = r_common_indices.iter().map(|&i| row[i].clone()).collect();
        r_index.entry(key).or_default().push(row);
    }

    // Probe with S
    let mut tuples = BTreeSet::new();
    for s_row in &s.tuples {
        let key: Tuple = s_common_indices.iter().map(|&i| s_row[i].clone()).collect();
        if let Some(r_rows) = r_index.get(&key) {
            for r_row in r_rows {
                let mut merged = (*r_row).clone();
                for &i in &s_extra_indices {
                    merged.push(s_row[i].clone());
                }
                tuples.insert(merged);
            }
        }
    }

    Relation {
        name: name.to_string(),
        columns,
        tuples,
    }
}

/// 2.1.4 Composition — `R·S` = projection of `natural_join(R, S)` onto non-joining columns.
fn composition(r: &Relation, s: &Relation, name: &str) -> Relation {
    let join = natural_join(r, s, "_tmp_join");

    // Find indices of the joining (common) columns in the join result
    let common_cols: Vec<String> = r
        .columns
        .iter()
        .filter(|c| s.columns.contains(c))
        .cloned()
        .collect();

    let non_common_indices: Vec<usize> = (0..join.columns.len())
        .filter(|i| !common_cols.contains(&join.columns[*i]))
        .collect();

    projection(&join, &non_common_indices, name)
}

/// 2.1.5 Restriction — Keep rows of R where columns `r_cols` match some row in S at `s_cols`.
///
/// # Panics
/// Panics if column index lists have different lengths or indices are out of bounds.
fn restriction(
    r: &Relation,
    s: &Relation,
    r_cols: &[usize],
    s_cols: &[usize],
    name: &str,
) -> Relation {
    assert_eq!(r_cols.len(), s_cols.len(), "Column lists must be same length");
    let r_degree = r.columns.len();
    let s_degree = s.columns.len();
    for &i in r_cols {
        assert!(i < r_degree, "R column index {i} out of bounds (degree {r_degree})");
    }
    for &i in s_cols {
        assert!(i < s_degree, "S column index {i} out of bounds (degree {s_degree})");
    }

    // Build set of matching key tuples from S
    let s_keys: BTreeSet<Tuple> = s
        .tuples
        .iter()
        .map(|row| s_cols.iter().map(|&i| row[i].clone()).collect())
        .collect();

    let tuples: BTreeSet<Tuple> = r
        .tuples
        .iter()
        .filter(|row| {
            let key: Tuple = r_cols.iter().map(|&i| row[i].clone()).collect();
            s_keys.contains(&key)
        })
        .cloned()
        .collect();

    Relation {
        name: name.to_string(),
        columns: r.columns.clone(),
        tuples,
    }
}

// =============================================================================
// Helper for verification
// =============================================================================

fn verify(label: &str, actual: &Relation, expected: &BTreeSet<Tuple>) -> bool {
    if actual.tuples == *expected {
        println!("  ✓ {label}");
        true
    } else {
        println!("  ✗ {label}");
        println!(
            "    Expected {} tuples, got {}",
            expected.len(),
            actual.tuples.len()
        );
        let missing: Vec<_> = expected.difference(&actual.tuples).collect();
        let extra: Vec<_> = actual.tuples.difference(expected).collect();
        if !missing.is_empty() {
            println!("    Missing: {missing:?}");
        }
        if !extra.is_empty() {
            println!("    Extra:   {extra:?}");
        }
        false
    }
}

/// Build a `BTreeSet` from a vec of tuples.
fn tset(rows: Vec<Tuple>) -> BTreeSet<Tuple> {
    rows.into_iter().collect()
}

// =============================================================================
// Data Definitions — All relations from the paper
// =============================================================================

fn build_supply() -> Relation {
    relation(
        "supply",
        &["supplier", "part", "project", "quantity"],
        vec![
            vec![int(1), int(2), int(5), int(17)],
            vec![int(1), int(3), int(5), int(23)],
            vec![int(2), int(3), int(7), int(9)],
            vec![int(2), int(7), int(5), int(4)],
            vec![int(4), int(1), int(1), int(12)],
        ],
    )
}

fn build_component() -> Relation {
    relation(
        "component",
        &["sub_part", "assembly", "quantity"],
        vec![
            vec![int(1), int(5), int(9)],
            vec![int(2), int(5), int(7)],
            vec![int(3), int(5), int(2)],
            vec![int(2), int(6), int(12)],
            vec![int(3), int(6), int(3)],
            vec![int(2), int(7), int(1)],
            vec![int(6), int(7), int(1)],
        ],
    )
}

fn build_employee() -> Relation {
    relation(
        "employee",
        &["man_no", "name", "birthdate"],
        vec![
            vec![int(1001), val("John Smith"), val("1935-04-12")],
            vec![int(1002), val("Mary Johnson"), val("1940-08-23")],
            vec![int(1003), val("Robert Brown"), val("1938-01-15")],
            vec![int(1004), val("Alice Williams"), val("1942-11-30")],
            vec![int(1005), val("James Davis"), val("1937-06-07")],
        ],
    )
}

fn build_jobhistory() -> Relation {
    relation(
        "jobhistory",
        &["man_no", "jobdate", "title"],
        vec![
            vec![int(1001), val("1960-01-15"), val("Junior Engineer")],
            vec![int(1001), val("1963-06-01"), val("Senior Engineer")],
            vec![int(1001), val("1968-03-15"), val("Project Lead")],
            vec![int(1002), val("1962-09-01"), val("Analyst")],
            vec![int(1002), val("1966-04-01"), val("Senior Analyst")],
            vec![int(1003), val("1959-07-01"), val("Technician")],
            vec![int(1003), val("1964-01-01"), val("Senior Technician")],
            vec![int(1004), val("1965-03-01"), val("Programmer")],
            vec![int(1005), val("1958-11-01"), val("Draftsman")],
            vec![int(1005), val("1963-02-01"), val("Designer")],
        ],
    )
}

fn build_salaryhistory() -> Relation {
    relation(
        "salaryhistory",
        &["man_no", "jobdate", "salarydate", "salary"],
        vec![
            vec![int(1001), val("1960-01-15"), val("1960-01-15"), int(8000)],
            vec![int(1001), val("1960-01-15"), val("1961-06-01"), int(8500)],
            vec![int(1001), val("1963-06-01"), val("1963-06-01"), int(11000)],
            vec![int(1001), val("1963-06-01"), val("1965-01-01"), int(12500)],
            vec![int(1001), val("1968-03-15"), val("1968-03-15"), int(16000)],
            vec![int(1002), val("1962-09-01"), val("1962-09-01"), int(9000)],
            vec![int(1002), val("1966-04-01"), val("1966-04-01"), int(13000)],
            vec![int(1003), val("1959-07-01"), val("1959-07-01"), int(7000)],
            vec![int(1003), val("1964-01-01"), val("1964-01-01"), int(9500)],
            vec![int(1004), val("1965-03-01"), val("1965-03-01"), int(10000)],
            vec![int(1005), val("1958-11-01"), val("1958-11-01"), int(6500)],
            vec![int(1005), val("1963-02-01"), val("1963-02-01"), int(9000)],
        ],
    )
}

fn build_children() -> Relation {
    relation(
        "children",
        &["man_no", "childname", "birthyear"],
        vec![
            vec![int(1001), val("Tom"), int(1962)],
            vec![int(1001), val("Susan"), int(1965)],
            vec![int(1002), val("Anna"), int(1964)],
            vec![int(1003), val("Peter"), int(1961)],
            vec![int(1003), val("Linda"), int(1963)],
            vec![int(1003), val("David"), int(1967)],
            vec![int(1005), val("Carol"), int(1960)],
        ],
    )
}

fn build_fig5_r() -> Relation {
    relation(
        "R",
        &["supplier", "part"],
        vec![
            vec![int(1), int(1)],
            vec![int(2), int(1)],
            vec![int(2), int(2)],
        ],
    )
}

fn build_fig5_s() -> Relation {
    relation(
        "S",
        &["part", "project"],
        vec![
            vec![int(1), int(1)],
            vec![int(1), int(2)],
            vec![int(2), int(1)],
        ],
    )
}

fn build_fig8_r() -> Relation {
    relation(
        "R",
        &["s", "p"],
        vec![
            vec![val("1"), val("a")],
            vec![val("2"), val("a")],
            vec![val("2"), val("b")],
        ],
    )
}

fn build_fig8_s() -> Relation {
    relation(
        "S",
        &["p", "j"],
        vec![
            vec![val("a"), val("d")],
            vec![val("a"), val("e")],
            vec![val("b"), val("d")],
            vec![val("b"), val("e")],
        ],
    )
}

fn build_fig8_t() -> Relation {
    relation(
        "T",
        &["j", "s"],
        vec![
            vec![val("d"), val("1")],
            vec![val("d"), val("2")],
            vec![val("e"), val("2")],
        ],
    )
}

fn build_fig12_r() -> Relation {
    relation(
        "R",
        &["supplier", "part"],
        vec![
            vec![val("1"), val("a")],
            vec![val("1"), val("b")],
            vec![val("1"), val("c")],
            vec![val("2"), val("c")],
            vec![val("2"), val("d")],
            vec![val("2"), val("e")],
        ],
    )
}

fn build_fig12_s() -> Relation {
    relation(
        "S",
        &["part", "project"],
        vec![
            vec![val("a"), val("g")],
            vec![val("b"), val("f")],
            vec![val("c"), val("f")],
            vec![val("c"), val("g")],
            vec![val("d"), val("g")],
            vec![val("e"), val("f")],
        ],
    )
}

fn build_fig13_r() -> Relation {
    relation(
        "R",
        &["s", "p", "j"],
        vec![
            vec![val("1"), val("a"), val("A")],
            vec![val("2"), val("a"), val("A")],
            vec![val("2"), val("a"), val("B")],
            vec![val("2"), val("b"), val("A")],
            vec![val("2"), val("b"), val("B")],
        ],
    )
}

fn build_fig13_s() -> Relation {
    relation(
        "S",
        &["p", "j"],
        vec![
            vec![val("a"), val("A")],
            vec![val("c"), val("B")],
            vec![val("b"), val("B")],
        ],
    )
}

// =============================================================================
// Demonstration helpers
// =============================================================================

fn demo_data(passed: &mut u32, total: &mut u32) {
    // Figure 1
    println!("═══ Figure 1: supply — relation of degree 4 ═══");
    let supply = build_supply();
    println!("{supply}\n");

    // Figure 2
    println!("═══ Figure 2: component — parts explosion ═══");
    let component = build_component();
    println!("{component}\n");

    // Figure 3(b)
    println!("═══ Figure 3(b): Normalized employee relations ═══");
    println!("{}\n", build_employee());
    println!("{}\n", build_jobhistory());
    println!("{}\n", build_salaryhistory());
    println!("{}\n", build_children());

    // Figure 4: Permuted projection
    println!("═══ Figure 4: Permuted projection of supply ═══");
    let fig4 = projection(&supply, &[2, 0], "Π_a(supply)");
    println!("{fig4}\n");

    *total += 1;
    if verify(
        "Fig 4: permuted projection",
        &fig4,
        &tset(vec![
            vec![int(1), int(4)],
            vec![int(5), int(1)],
            vec![int(5), int(2)],
            vec![int(7), int(2)],
        ]),
    ) {
        *passed += 1;
    }

    // Permutation (Section 2.1.1)
    println!("\n═══ Section 2.1.1: Permutation of supply ═══");
    let supply_perm = permutation(
        &supply,
        &[2, 1, 0, 3],
        "supply (permuted: project, part, supplier, quantity)",
    );
    println!("{supply_perm}\n");

    *total += 1;
    if verify(
        "Permutation preserves all tuples (reordered)",
        &supply_perm,
        &tset(vec![
            vec![int(1), int(1), int(4), int(12)],
            vec![int(5), int(2), int(1), int(17)],
            vec![int(5), int(3), int(1), int(23)],
            vec![int(5), int(7), int(2), int(4)],
            vec![int(7), int(3), int(2), int(9)],
        ]),
    ) {
        *passed += 1;
    }
}

fn demo_joins(passed: &mut u32, total: &mut u32) {
    // Figure 5 & 6: Natural join
    println!("\n═══ Figure 5: Two joinable binary relations ═══");
    let fig5_r = build_fig5_r();
    let fig5_s = build_fig5_s();
    println!("{fig5_r}\n");
    println!("{fig5_s}\n");

    println!("═══ Figure 6: Natural join R*S ═══");
    let fig6 = natural_join(&fig5_r, &fig5_s, "R*S");
    println!("{fig6}\n");

    *total += 1;
    if verify(
        "Fig 6: natural join R*S",
        &fig6,
        &tset(vec![
            vec![int(1), int(1), int(1)],
            vec![int(1), int(1), int(2)],
            vec![int(2), int(1), int(1)],
            vec![int(2), int(1), int(2)],
            vec![int(2), int(2), int(1)],
        ]),
    ) {
        *passed += 1;
    }

    // Figure 7: Another join (non-natural)
    println!("\n═══ Figure 7: Another join of R with S ═══");
    println!("(An alternative join where π₁₂=R and π₂₃=S)");
    let fig7 = relation(
        "U",
        &["supplier", "part", "project"],
        vec![
            vec![int(1), int(1), int(2)],
            vec![int(2), int(1), int(1)],
            vec![int(2), int(2), int(1)],
        ],
    );
    println!("{fig7}\n");

    let fig7_p12 = projection(&fig7, &[0, 1], "π₁₂(U)");
    let fig7_p23 = projection(&fig7, &[1, 2], "π₂₃(U)");

    *total += 1;
    if verify("Fig 7: π₁₂(U) = R", &fig7_p12, &fig5_r.tuples) {
        *passed += 1;
    }
    *total += 1;
    if verify("Fig 7: π₂₃(U) = S", &fig7_p23, &fig5_s.tuples) {
        *passed += 1;
    }

    // Figure 8 & 9: Cyclic 3-join
    println!("\n═══ Figure 8: Binary relations for cyclic 3-join ═══");
    let fig8_r = build_fig8_r();
    let fig8_s = build_fig8_s();
    let fig8_t = build_fig8_t();
    println!("{fig8_r}\n");
    println!("{fig8_s}\n");
    println!("{fig8_t}\n");

    println!("═══ Figure 9: Cyclic 3-join (U' = R ⋈ S ⋈ T) ═══");
    let rs = natural_join(&fig8_r, &fig8_s, "_RS");
    let fig9 = natural_join(&rs, &fig8_t, "U'");
    println!("{fig9}\n");

    *total += 1;
    if verify(
        "Fig 9: cyclic 3-join U'",
        &fig9,
        &tset(vec![
            vec![val("1"), val("a"), val("d")],
            vec![val("2"), val("a"), val("d")],
            vec![val("2"), val("a"), val("e")],
            vec![val("2"), val("b"), val("d")],
            vec![val("2"), val("b"), val("e")],
        ]),
    ) {
        *passed += 1;
    }

    // Verify projections of U' equal original relations
    let u_sp = projection(&fig9, &[0, 1], "π_sp(U')");
    let u_pj = projection(&fig9, &[1, 2], "π_pj(U')");
    let u_js = projection(&fig9, &[2, 0], "π_js(U')");

    *total += 1;
    if verify("Fig 9: π_sp(U') = R", &u_sp, &fig8_r.tuples) {
        *passed += 1;
    }
    *total += 1;
    if verify("Fig 9: π_pj(U') = S", &u_pj, &fig8_s.tuples) {
        *passed += 1;
    }
    *total += 1;
    if verify("Fig 9: π_js(U') = T", &u_js, &fig8_t.tuples) {
        *passed += 1;
    }
}

fn demo_composition_restriction(passed: &mut u32, total: &mut u32) {
    let fig5_r = build_fig5_r();
    let fig5_s = build_fig5_s();

    // Figure 10: Natural composition
    println!("\n═══ Figure 10: Natural composition R·S ═══");
    let fig10 = composition(&fig5_r, &fig5_s, "R·S");
    println!("{fig10}\n");

    *total += 1;
    if verify(
        "Fig 10: composition R·S",
        &fig10,
        &tset(vec![
            vec![int(1), int(1)],
            vec![int(1), int(2)],
            vec![int(2), int(1)],
            vec![int(2), int(2)],
        ]),
    ) {
        *passed += 1;
    }

    // Figure 11: Another composition (from U in Fig 7)
    println!("═══ Figure 11: Another composition (from U in Fig 7) ═══");
    let fig7 = relation(
        "U",
        &["supplier", "part", "project"],
        vec![
            vec![int(1), int(1), int(2)],
            vec![int(2), int(1), int(1)],
            vec![int(2), int(2), int(1)],
        ],
    );
    let fig11 = projection(&fig7, &[0, 2], "T");
    println!("{fig11}\n");

    *total += 1;
    if verify(
        "Fig 11: composition from U",
        &fig11,
        &tset(vec![vec![int(1), int(2)], vec![int(2), int(1)]]),
    ) {
        *passed += 1;
    }

    // Figure 12: Many joins, only one composition
    println!("═══ Figure 12: Many joins, only one composition ═══");
    let fig12_r = build_fig12_r();
    let fig12_s = build_fig12_s();
    println!("{fig12_r}\n");
    println!("{fig12_s}\n");

    let fig12_join = natural_join(&fig12_r, &fig12_s, "R*S");
    println!("Natural join R*S:");
    println!("{fig12_join}\n");

    let fig12_comp = composition(&fig12_r, &fig12_s, "R·S");
    println!("Composition R·S:");
    println!("{fig12_comp}\n");

    *total += 1;
    if verify(
        "Fig 12: composition R·S = {(1,f),(1,g),(2,f),(2,g)}",
        &fig12_comp,
        &tset(vec![
            vec![val("1"), val("f")],
            vec![val("1"), val("g")],
            vec![val("2"), val("f")],
            vec![val("2"), val("g")],
        ]),
    ) {
        *passed += 1;
    }

    // Figure 13: Restriction
    println!("\n═══ Figure 13: Restriction R'= R|S on (p, j) ═══");
    let fig13_r = build_fig13_r();
    let fig13_s = build_fig13_s();
    println!("{fig13_r}\n");
    println!("{fig13_s}\n");

    let fig13_result = restriction(&fig13_r, &fig13_s, &[1, 2], &[0, 1], "R'");
    println!("Restriction result R':");
    println!("{fig13_result}\n");

    *total += 1;
    if verify(
        "Fig 13: restriction R'",
        &fig13_result,
        &tset(vec![
            vec![val("1"), val("a"), val("A")],
            vec![val("2"), val("a"), val("A")],
            vec![val("2"), val("b"), val("B")],
        ]),
    ) {
        *passed += 1;
    }
}

// =============================================================================
// Main
// =============================================================================

fn main() {
    let mut passed: u32 = 0;
    let mut total: u32 = 0;

    demo_data(&mut passed, &mut total);
    demo_joins(&mut passed, &mut total);
    demo_composition_restriction(&mut passed, &mut total);

    println!("\n{}", "=".repeat(60));
    println!("Verification: {passed}/{total} passed");
    if passed == total {
        println!("All verifications passed.");
    } else {
        println!("SOME VERIFICATIONS FAILED.");
        std::process::exit(1);
    }
}
