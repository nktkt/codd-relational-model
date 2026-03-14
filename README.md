# Codd's Relational Model

Two implementations of all relations, sample data, and relational operations from E.F. Codd's foundational 1970 paper:

> **"A Relational Model of Data for Large Shared Data Banks"**
> E. F. Codd, IBM Research Laboratory, San Jose, California
> *Communications of the ACM*, Volume 13, Number 6, June 1970, pp. 377–387

## Implementations

| | SQLite | Rust |
|---|--------|------|
| **File** | `codd_relational_model.sql` | `src/main.rs` |
| **Approach** | SQL tables & views | Pure relational algebra engine |
| **Dependencies** | sqlite3 | None (std only) |
| **Relations** | 29 tables + 5 views | 15 relations + computed results |
| **Operations** | Views (SQL) | Functions (projection, join, composition, restriction, permutation) |
| **Verification** | Manual query | 13 automated assertions |

## Quick Start

### Rust

```bash
cargo run
```

Output: all relations from the paper printed as tables, with each relational operation computed and verified against expected results (13/13 assertions).

```
═══ Figure 6: Natural join R*S ═══
R*S (5 tuples):
┌──────────┬──────┬─────────┐
│ supplier │ part │ project │
├──────────┼──────┼─────────┤
│ 1        │ 1    │ 1       │
│ 1        │ 1    │ 2       │
│ 2        │ 1    │ 1       │
│ 2        │ 1    │ 2       │
│ 2        │ 2    │ 1       │
└──────────┴──────┴─────────┘
  ✓ Fig 6: natural join R*S
```

### SQLite

```bash
sqlite3 codd_relational_model.db < codd_relational_model.sql

# Query the supply relation (Figure 1)
sqlite3 -header -column codd_relational_model.db "SELECT * FROM supply;"

# See the natural join in action (Figure 6)
sqlite3 -header -column codd_relational_model.db "SELECT * FROM fig6_natural_join;"

# Verify cyclic 3-join projections equal the original relations
sqlite3 -header -column codd_relational_model.db "
  SELECT DISTINCT s, p FROM fig9_cyclic_3join;   -- should equal fig8_R
  SELECT DISTINCT p, j FROM fig9_cyclic_3join;   -- should equal fig8_S
  SELECT DISTINCT j, s FROM fig9_cyclic_3join;   -- should equal fig8_T
"
```

## Rust: Relational Algebra Engine

### Core Types

```rust
enum Value { Int(i64), Str(String) }

struct Relation {
    name: String,
    columns: Vec<String>,
    tuples: BTreeSet<Vec<Value>>,  // set semantics — no duplicates
}
```

### Operations (Section 2.1)

| Function | Paper Section | Description |
|----------|--------------|-------------|
| `projection()` | 2.1.2 | Select columns by index, remove duplicates |
| `permutation()` | 2.1.1 | Reorder all columns |
| `natural_join()` | 2.1.3 | Join on all commonly-named columns |
| `composition()` | 2.1.4 | Project natural join onto non-joining columns |
| `restriction()` | 2.1.5 | Filter rows by matching against another relation |

### Verified Figures

| Verification | Operation | Result |
|-------------|-----------|--------|
| Fig 4 | Permuted projection of `supply` | 4 tuples |
| Fig 6 | Natural join R\*S | 5 tuples |
| Fig 7 | Alternative join — π₁₂(U)=R, π₂₃(U)=S | 3 tuples |
| Fig 9 | Cyclic 3-join R ⋈ S ⋈ T + 3 projection checks | 5 tuples |
| Fig 10 | Composition R·S | 4 tuples |
| Fig 11 | Composition from alternative join | 2 tuples |
| Fig 12 | Many joins, one composition | 4 tuples |
| Fig 13 | Restriction R' = R\|S on (p, j) | 3 tuples |
| Permutation | Reorder supply columns | 5 tuples |

## SQLite: Data & Views

### Tables

**Core (Section 1.3):** `supply`, `component`, `part`, `supplier`, `project`

**Normalized employees (Figure 3b):** `employee`, `jobhistory`, `salaryhistory`, `children`

**Redundancy examples (Section 2.2):** `employee_with_redundancy`, `S_supplier`, `D_department`, `J_project`, `P_supplies_dept`, `Q_supplies_proj`, `R_dept_proj`

**Operation inputs (Section 2.1):** `fig5_R`, `fig5_S`, `fig7_another_join_data`, `fig8_R`, `fig8_S`, `fig8_T`, `fig12_R`, `fig12_S`, `fig13_R`, `fig13_S`

**Metadata:** `paper_metadata`, `relational_concepts`, `hierarchical_structures`

### Views

| View | Figure | Operation |
|------|--------|-----------|
| `fig4_permuted_projection` | 4 | Permuted projection of `supply` |
| `fig6_natural_join` | 6 | Natural join R\*S |
| `fig9_cyclic_3join` | 9 | Cyclic 3-join |
| `fig10_natural_composition` | 10 | Composition R·S |
| `fig13_restriction_result` | 13 | Restriction R' = R\|S |

**29 tables**, **5 views** — with foreign key constraints (`PRAGMA foreign_keys = ON`) and UNIQUE constraints on all relation tables.

## References

- Codd, E. F. (1970). "A Relational Model of Data for Large Shared Data Banks." *Communications of the ACM*, 13(6), 377–387. [PDF](https://www.seas.upenn.edu/~zives/03f/cis550/codd.pdf)
