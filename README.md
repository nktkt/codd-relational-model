# Codd's Relational Model — SQLite Database

A SQLite database implementing all relations, sample data, and relational operations from E.F. Codd's foundational 1970 paper:

> **"A Relational Model of Data for Large Shared Data Banks"**
> E. F. Codd, IBM Research Laboratory, San Jose, California
> *Communications of the ACM*, Volume 13, Number 6, June 1970, pp. 377–387

## Contents

### Core Relations (Section 1.3 — Parts/Projects/Suppliers)

| Table | Description | Primary Key |
|-------|-------------|-------------|
| `part` | Parts with number, name, color, weight, inventory | `part_number` |
| `supplier` | Suppliers with name and city | `supplier_number` |
| `project` | Projects with name and description | `project_number` |
| `supply` | Shipments-in-progress (Figure 1, degree 4) | `(supplier_id, part_id, project_id)` |
| `component` | Parts explosion (Figure 2, degree 3) | `(sub_part, assembly)` |

### Normalized Employee Relations (Figure 3b)

Demonstrates Codd's normalization of nonsimple domains into first normal form.

| Table | Description | Primary Key |
|-------|-------------|-------------|
| `employee` | Employee master | `man_no` |
| `jobhistory` | Job assignments over time | `(man_no, jobdate)` |
| `salaryhistory` | Salary changes within each job | `(man_no, jobdate, salarydate)` |
| `children` | Employee's children | `(man_no, childname)` |

### Redundancy & Consistency Examples (Section 2.2)

| Table | Description |
|-------|-------------|
| `employee_with_redundancy` | Strong redundancy example — `managername` is derivable |
| `S_supplier`, `D_department`, `J_project` | Entity relations for weak redundancy example |
| `P_supplies_dept`, `Q_supplies_proj`, `R_dept_proj` | Relationship relations demonstrating redundancy |

### Relational Operations (Section 2.1)

| Table / View | Paper Figure | Operation |
|-------------|-------------|-----------|
| `fig5_R`, `fig5_S` | Figure 5 | Two joinable binary relations |
| `fig6_natural_join` (view) | Figure 6 | Natural join R\*S |
| `fig7_another_join_data` | Figure 7 | An alternative (non-natural) join of R with S |
| `fig4_permuted_projection` (view) | Figure 4 | Permuted projection of `supply` |
| `fig8_R`, `fig8_S`, `fig8_T` | Figure 8 | Binary relations for cyclic 3-joins |
| `fig9_cyclic_3join` (view) | Figure 9 | Cyclic 3-join |
| `fig10_natural_composition` (view) | Figure 10 | Natural composition R·S |
| `fig12_R`, `fig12_S` | Figure 12 | Many joins, only one composition |
| `fig13_R`, `fig13_S` | Figure 13 | Restriction example (input) |
| `fig13_restriction_result` (view) | Figure 13 | Restriction R' = R\|S |

### Metadata

| Table | Description |
|-------|-------------|
| `paper_metadata` | Bibliographic information |
| `relational_concepts` | 15 key concepts defined in the paper |
| `hierarchical_structures` | The 5 tree structures from Section 1.2.3 |

## Quick Start

```bash
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

# Enable foreign key enforcement (required per-connection in SQLite)
sqlite3 codd_relational_model.db "PRAGMA foreign_keys = ON; INSERT INTO supply VALUES (99,99,99,1);"
# → Error: FOREIGN KEY constraint failed
```

## Building from Source

```bash
sqlite3 codd_relational_model.db < codd_relational_model.sql
```

## Schema Summary

- **29 tables**, **5 views**
- Foreign key constraints with `PRAGMA foreign_keys = ON`
- UNIQUE constraints on all relation tables (relations are sets — no duplicate tuples)
- Views implement projection, natural join, cyclic 3-join, composition, and restriction

## References

- Codd, E. F. (1970). "A Relational Model of Data for Large Shared Data Banks." *Communications of the ACM*, 13(6), 377–387. [PDF](https://www.seas.upenn.edu/~zives/03f/cis550/codd.pdf)
