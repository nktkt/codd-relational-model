-- ============================================================================
-- Database based on E.F. Codd's 1970 paper:
-- "A Relational Model of Data for Large Shared Data Banks"
-- Communications of the ACM, Volume 13, Number 6, June 1970
-- ============================================================================

-- Enable foreign key enforcement (SQLite does not enforce by default)
PRAGMA foreign_keys = ON;

-- ============================================================================
-- SECTION 1.3: The Parts/Projects/Suppliers Data Bank
-- ============================================================================

-- The "part" relation - defined on domains described in Section 1.3
-- Domains: part_number, part_name, part_color, part_weight,
--          quantity_on_hand, quantity_on_order
CREATE TABLE part (
    part_number  INTEGER PRIMARY KEY,
    part_name    TEXT NOT NULL,
    part_color   TEXT,
    part_weight  REAL,
    quantity_on_hand  INTEGER DEFAULT 0,
    quantity_on_order INTEGER DEFAULT 0
);

INSERT INTO part (part_number, part_name, part_color, part_weight, quantity_on_hand, quantity_on_order) VALUES
    (1, 'Bolt',    'Red',    0.5,  900, 100),
    (2, 'Nut',     'Green',  0.3,  500, 200),
    (3, 'Screw',   'Blue',   0.4,  750, 150),
    (5, 'Cam',     'Blue',   1.2,  300,  50),
    (6, 'Cog',     'Red',    2.0,  200,  80),
    (7, 'Washer',  'Silver', 0.1, 1200, 300);

-- Supplier entity
CREATE TABLE supplier (
    supplier_number INTEGER PRIMARY KEY,
    supplier_name   TEXT NOT NULL,
    city            TEXT
);

INSERT INTO supplier (supplier_number, supplier_name, city) VALUES
    (1, 'Smith',  'London'),
    (2, 'Jones',  'Paris'),
    (3, 'Blake',  'Paris'),
    (4, 'Clark',  'London');

-- Project entity
CREATE TABLE project (
    project_number      INTEGER PRIMARY KEY,
    project_name        TEXT NOT NULL,
    project_description TEXT
);

INSERT INTO project (project_number, project_name, project_description) VALUES
    (1, 'Alpha', 'Instrumentation project'),
    (5, 'Beta',  'Engine assembly project'),
    (6, 'Gamma', 'Navigation system project'),
    (7, 'Delta', 'Communications project');

-- ============================================================================
-- FIGURE 1: The "supply" relation (degree 4)
-- Reflects shipments-in-progress of parts from specified suppliers
-- to specified projects in specified quantities.
-- Primary key: (supplier, part, project)
-- Each of supplier, part, project is individually a foreign key.
-- ============================================================================
CREATE TABLE supply (
    supplier_id INTEGER NOT NULL,
    part_id     INTEGER NOT NULL,
    project_id  INTEGER NOT NULL,
    quantity    INTEGER NOT NULL,
    PRIMARY KEY (supplier_id, part_id, project_id),
    FOREIGN KEY (supplier_id) REFERENCES supplier(supplier_number),
    FOREIGN KEY (part_id)     REFERENCES part(part_number),
    FOREIGN KEY (project_id)  REFERENCES project(project_number)
);

INSERT INTO supply (supplier_id, part_id, project_id, quantity) VALUES
    (1, 2, 5, 17),
    (1, 3, 5, 23),
    (2, 3, 7,  9),
    (2, 7, 5,  4),
    (4, 1, 1, 12);

-- ============================================================================
-- FIGURE 2: The "component" relation (degree 3)
-- component(x, y, z) means: part x is an immediate component
-- (or subassembly) of part y, and z units of part x are needed
-- to assemble one unit of part y.
-- Plays a critical role in the parts explosion problem.
-- Two domains are both "part" but with distinct roles.
-- ============================================================================
CREATE TABLE component (
    sub_part    INTEGER NOT NULL,
    assembly    INTEGER NOT NULL,
    quantity    INTEGER NOT NULL,
    PRIMARY KEY (sub_part, assembly),
    FOREIGN KEY (sub_part) REFERENCES part(part_number),
    FOREIGN KEY (assembly) REFERENCES part(part_number)
);

INSERT INTO component (sub_part, assembly, quantity) VALUES
    (1, 5,  9),
    (2, 5,  7),
    (3, 5,  2),
    (2, 6, 12),
    (3, 6,  3),
    (2, 7,  1),
    (6, 7,  1);

-- ============================================================================
-- FIGURE 3(b): Normalized employee relations
-- Derived from the unnormalized set in Figure 3(a) via normalization.
-- Original unnormalized relation: employee(man#, name, birthdate,
--   jobhistory(jobdate, title, salaryhistory(salarydate, salary)),
--   children(childname, birthyear))
-- Primary keys shown in italics in the paper are underlined here.
-- ============================================================================

-- employee' relation - primary key: man_no
CREATE TABLE employee (
    man_no    INTEGER PRIMARY KEY,
    name      TEXT NOT NULL,
    birthdate TEXT
);

INSERT INTO employee (man_no, name, birthdate) VALUES
    (1001, 'John Smith',    '1935-04-12'),
    (1002, 'Mary Johnson',  '1940-08-23'),
    (1003, 'Robert Brown',  '1938-01-15'),
    (1004, 'Alice Williams', '1942-11-30'),
    (1005, 'James Davis',   '1937-06-07');

-- jobhistory' relation - primary key: (man_no, jobdate)
CREATE TABLE jobhistory (
    man_no  INTEGER NOT NULL,
    jobdate TEXT NOT NULL,
    title   TEXT NOT NULL,
    PRIMARY KEY (man_no, jobdate),
    FOREIGN KEY (man_no) REFERENCES employee(man_no)
);

INSERT INTO jobhistory (man_no, jobdate, title) VALUES
    (1001, '1960-01-15', 'Junior Engineer'),
    (1001, '1963-06-01', 'Senior Engineer'),
    (1001, '1968-03-15', 'Project Lead'),
    (1002, '1962-09-01', 'Analyst'),
    (1002, '1966-04-01', 'Senior Analyst'),
    (1003, '1959-07-01', 'Technician'),
    (1003, '1964-01-01', 'Senior Technician'),
    (1004, '1965-03-01', 'Programmer'),
    (1005, '1958-11-01', 'Draftsman'),
    (1005, '1963-02-01', 'Designer');

-- salaryhistory' relation - primary key: (man_no, jobdate, salarydate)
CREATE TABLE salaryhistory (
    man_no     INTEGER NOT NULL,
    jobdate    TEXT NOT NULL,
    salarydate TEXT NOT NULL,
    salary     REAL NOT NULL,
    PRIMARY KEY (man_no, jobdate, salarydate),
    FOREIGN KEY (man_no, jobdate) REFERENCES jobhistory(man_no, jobdate)
);

INSERT INTO salaryhistory (man_no, jobdate, salarydate, salary) VALUES
    (1001, '1960-01-15', '1960-01-15', 8000),
    (1001, '1960-01-15', '1961-06-01', 8500),
    (1001, '1963-06-01', '1963-06-01', 11000),
    (1001, '1963-06-01', '1965-01-01', 12500),
    (1001, '1968-03-15', '1968-03-15', 16000),
    (1002, '1962-09-01', '1962-09-01', 9000),
    (1002, '1966-04-01', '1966-04-01', 13000),
    (1003, '1959-07-01', '1959-07-01', 7000),
    (1003, '1964-01-01', '1964-01-01', 9500),
    (1004, '1965-03-01', '1965-03-01', 10000),
    (1005, '1958-11-01', '1958-11-01', 6500),
    (1005, '1963-02-01', '1963-02-01', 9000);

-- children' relation - primary key: (man_no, childname)
CREATE TABLE children (
    man_no    INTEGER NOT NULL,
    childname TEXT NOT NULL,
    birthyear INTEGER NOT NULL,
    PRIMARY KEY (man_no, childname),
    FOREIGN KEY (man_no) REFERENCES employee(man_no)
);

INSERT INTO children (man_no, childname, birthyear) VALUES
    (1001, 'Tom',   1962),
    (1001, 'Susan', 1965),
    (1002, 'Anna',  1964),
    (1003, 'Peter', 1961),
    (1003, 'Linda', 1963),
    (1003, 'David', 1967),
    (1005, 'Carol', 1960);

-- ============================================================================
-- SECTION 2.2.1: Strong Redundancy Example
-- employee(serial#, name, manager#, managername)
-- serial# is primary key, manager# is a foreign key.
-- managername is strongly redundant because:
--   active_domain(manager#) ⊂ active_domain(serial#)
--   active_domain(managername) ⊂ active_domain(name)
-- ============================================================================
CREATE TABLE employee_with_redundancy (
    serial_no    INTEGER PRIMARY KEY,
    name         TEXT NOT NULL,
    manager_no   INTEGER,
    managername  TEXT,
    FOREIGN KEY (manager_no) REFERENCES employee_with_redundancy(serial_no)
);

INSERT INTO employee_with_redundancy (serial_no, name, manager_no, managername) VALUES
    (100, 'Alice Williams', NULL, NULL),
    (101, 'John Smith',     100,  'Alice Williams'),
    (102, 'Mary Johnson',   100,  'Alice Williams'),
    (103, 'Robert Brown',   101,  'John Smith'),
    (104, 'James Davis',    101,  'John Smith'),
    (105, 'Carol White',    102,  'Mary Johnson');

-- ============================================================================
-- SECTION 2.2: Redundancy - S, J, D, P, Q, R relations
-- S: suppliers (primary key s#)
-- D: departments (primary key d#)
-- J: projects (primary key j#)
-- P(s#, d#, ...): supplier s supplies department d
-- Q(s#, j#, ...): supplier s supplies project j
-- R_dept(d#, j#, ...): department d is assigned to project j
-- These demonstrate weak and strong redundancy.
-- ============================================================================
CREATE TABLE S_supplier (
    s_no   INTEGER PRIMARY KEY,
    s_name TEXT NOT NULL
);

INSERT INTO S_supplier (s_no, s_name) VALUES
    (1, 'Acme Corp'),
    (2, 'Beta Inc'),
    (3, 'Gamma Ltd'),
    (4, 'Delta Co');

CREATE TABLE D_department (
    d_no   INTEGER PRIMARY KEY,
    d_name TEXT NOT NULL
);

INSERT INTO D_department (d_no, d_name) VALUES
    (1, 'Engineering'),
    (2, 'Manufacturing'),
    (3, 'Research'),
    (5, 'Quality Assurance');

CREATE TABLE J_project (
    j_no   INTEGER PRIMARY KEY,
    j_name TEXT NOT NULL
);

INSERT INTO J_project (j_no, j_name) VALUES
    (1, 'Project X'),
    (2, 'Project Y'),
    (3, 'Project Z');

-- P: supplier s supplies department d (if and only if)
CREATE TABLE P_supplies_dept (
    s_no INTEGER NOT NULL,
    d_no INTEGER NOT NULL,
    PRIMARY KEY (s_no, d_no),
    FOREIGN KEY (s_no) REFERENCES S_supplier(s_no),
    FOREIGN KEY (d_no) REFERENCES D_department(d_no)
);

INSERT INTO P_supplies_dept (s_no, d_no) VALUES
    (1, 1),
    (1, 2),
    (2, 2),
    (2, 5),
    (3, 1),
    (4, 3);

-- Q: supplier s supplies project j
CREATE TABLE Q_supplies_proj (
    s_no INTEGER NOT NULL,
    j_no INTEGER NOT NULL,
    PRIMARY KEY (s_no, j_no),
    FOREIGN KEY (s_no) REFERENCES S_supplier(s_no),
    FOREIGN KEY (j_no) REFERENCES J_project(j_no)
);

INSERT INTO Q_supplies_proj (s_no, j_no) VALUES
    (1, 1),
    (1, 2),
    (2, 1),
    (2, 3),
    (3, 2),
    (4, 3);

-- R_dept_proj: department d is assigned to project j
CREATE TABLE R_dept_proj (
    d_no INTEGER NOT NULL,
    j_no INTEGER NOT NULL,
    PRIMARY KEY (d_no, j_no),
    FOREIGN KEY (d_no) REFERENCES D_department(d_no),
    FOREIGN KEY (j_no) REFERENCES J_project(j_no)
);

INSERT INTO R_dept_proj (d_no, j_no) VALUES
    (1, 1),
    (1, 2),
    (2, 1),
    (2, 3),
    (3, 2),
    (5, 3);

-- ============================================================================
-- FIGURE 5: Two joinable binary relations R and S
-- Used to demonstrate natural join (Figure 6), another join (Figure 7)
-- ============================================================================
CREATE TABLE fig5_R (
    supplier INTEGER NOT NULL,
    part     INTEGER NOT NULL,
    UNIQUE (supplier, part)
);

INSERT INTO fig5_R (supplier, part) VALUES
    (1, 1),
    (2, 1),
    (2, 2);

CREATE TABLE fig5_S (
    part    INTEGER NOT NULL,
    project INTEGER NOT NULL,
    UNIQUE (part, project)
);

INSERT INTO fig5_S (part, project) VALUES
    (1, 1),
    (1, 2),
    (2, 1);

-- ============================================================================
-- FIGURE 8: Binary relations with a plurality of cyclic 3-joins
-- R(s, p), S(p, j), T(j, s)
-- ============================================================================
CREATE TABLE fig8_R (
    s TEXT NOT NULL,
    p TEXT NOT NULL,
    UNIQUE (s, p)
);

INSERT INTO fig8_R (s, p) VALUES
    ('1', 'a'),
    ('2', 'a'),
    ('2', 'b');

CREATE TABLE fig8_S (
    p TEXT NOT NULL,
    j TEXT NOT NULL,
    UNIQUE (p, j)
);

-- Note: The first row's j value appears as 'A' in the scan but must be 'd'
-- because: (1) the j domain must match fig8_T's j values {d, e}, and
-- (2) the paper text states "the points x = a; y = d; z = 2" confirming S(a, d).
INSERT INTO fig8_S (p, j) VALUES
    ('a', 'd'),
    ('a', 'e'),
    ('b', 'd'),
    ('b', 'e');

CREATE TABLE fig8_T (
    j TEXT NOT NULL,
    s TEXT NOT NULL,
    UNIQUE (j, s)
);

INSERT INTO fig8_T (j, s) VALUES
    ('d', '1'),
    ('d', '2'),
    ('e', '2');

-- ============================================================================
-- FIGURE 12: Many joins, only one composition
-- Demonstrates that ambiguity of the join domain is lost in composition
-- ============================================================================
CREATE TABLE fig12_R (
    supplier TEXT NOT NULL,
    part     TEXT NOT NULL,
    UNIQUE (supplier, part)
);

INSERT INTO fig12_R (supplier, part) VALUES
    ('1', 'a'),
    ('1', 'b'),
    ('1', 'c'),
    ('2', 'c'),
    ('2', 'd'),
    ('2', 'e');

CREATE TABLE fig12_S (
    part    TEXT NOT NULL,
    project TEXT NOT NULL,
    UNIQUE (part, project)
);

-- Note: The last row's part value appears as 'a' in the scan but must be 'e'
-- because: (1) part 'e' exists in fig12_R but would have no join partner, and
-- (2) the paper text states "unambiguous associations made via points a, b, d, e"
-- confirming that 'e' must exist in S.
INSERT INTO fig12_S (part, project) VALUES
    ('a', 'g'),
    ('b', 'f'),
    ('c', 'f'),
    ('c', 'g'),
    ('d', 'g'),
    ('e', 'f');

-- ============================================================================
-- FIGURE 13: Example of Restriction
-- R' = R restricted by S on columns (p, j)
-- R' is the maximal subset of R such that π_L(R') = π_M(S)
-- ============================================================================
CREATE TABLE fig13_R (
    s TEXT NOT NULL,
    p TEXT NOT NULL,
    j TEXT NOT NULL,
    UNIQUE (s, p, j)
);

INSERT INTO fig13_R (s, p, j) VALUES
    ('1', 'a', 'A'),
    ('2', 'a', 'A'),
    ('2', 'a', 'B'),
    ('2', 'b', 'A'),
    ('2', 'b', 'B');

CREATE TABLE fig13_S (
    p TEXT NOT NULL,
    j TEXT NOT NULL,
    UNIQUE (p, j)
);

INSERT INTO fig13_S (p, j) VALUES
    ('a', 'A'),
    ('c', 'B'),
    ('b', 'B');

-- ============================================================================
-- VIEWS: Demonstrate relational operations described in the paper
-- ============================================================================

-- Figure 4: Permuted projection of supply (project, supplier)
CREATE VIEW fig4_permuted_projection AS
SELECT DISTINCT project_id AS project, supplier_id AS supplier
FROM supply;

-- Figure 6: Natural join of R with S (from Figure 5)
CREATE VIEW fig6_natural_join AS
SELECT R.supplier, R.part, S.project
FROM fig5_R R
JOIN fig5_S S ON R.part = S.part;

-- Figure 7: Another join of R with S (from Figure 5)
-- U is an alternative join (not the natural join) such that
-- π_12(U) = R and π_23(U) = S (i.e. U projects back to both R and S).
-- This is the minimal join where each part value maps suppliers to projects
-- as a bijection rather than a Cartesian product.
-- Paper shows: U = {(1,1,2), (2,1,1), (2,2,1)}
CREATE TABLE fig7_another_join_data (
    supplier INTEGER NOT NULL,
    part     INTEGER NOT NULL,
    project  INTEGER NOT NULL,
    UNIQUE (supplier, part, project)
);

INSERT INTO fig7_another_join_data (supplier, part, project) VALUES
    (1, 1, 2),
    (2, 1, 1),
    (2, 2, 1);

-- Verify: π_12 = R, π_23 = S
-- SELECT DISTINCT supplier, part FROM fig7_another_join_data;  -- should equal fig5_R
-- SELECT DISTINCT part, project FROM fig7_another_join_data;   -- should equal fig5_S

-- Figure 9: Cyclic 3-joins of relations in Figure 8
CREATE VIEW fig9_cyclic_3join AS
SELECT R.s, R.p, S.j
FROM fig8_R R
JOIN fig8_S S ON R.p = S.p
JOIN fig8_T T ON S.j = T.j AND T.s = R.s;

-- Figure 10: Natural composition R·S (from Figure 5)
-- R·S = π_13(R*S) - projection of natural join onto 1st and 3rd columns
CREATE VIEW fig10_natural_composition AS
SELECT DISTINCT R.supplier AS supplier, S.project AS project
FROM fig5_R R
JOIN fig5_S S ON R.part = S.part;

-- Figure 13 result: Restriction of R by S on columns (p, j)
CREATE VIEW fig13_restriction_result AS
SELECT R.s, R.p, R.j
FROM fig13_R R
WHERE EXISTS (
    SELECT 1 FROM fig13_S S
    WHERE S.p = R.p AND S.j = R.j
);

-- ============================================================================
-- Metadata: Paper information
-- ============================================================================
CREATE TABLE paper_metadata (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

INSERT INTO paper_metadata (key, value) VALUES
    ('title',       'A Relational Model of Data for Large Shared Data Banks'),
    ('author',      'E. F. Codd'),
    ('affiliation', 'IBM Research Laboratory, San Jose, California'),
    ('journal',     'Communications of the ACM'),
    ('volume',      '13'),
    ('number',      '6'),
    ('date',        'June 1970'),
    ('pages',       '377-387'),
    ('received',    'September 1969'),
    ('revised',     'February 1970');

-- ============================================================================
-- Metadata: The 5 hierarchical structures (Section 1.2.3)
-- These illustrate tree-structured representations of the same data
-- about parts and projects, showing access path dependence problems.
-- ============================================================================
CREATE TABLE hierarchical_structures (
    structure_id  INTEGER PRIMARY KEY,
    name          TEXT NOT NULL,
    description   TEXT NOT NULL,
    file_f        TEXT NOT NULL,
    file_g        TEXT,
    file_h        TEXT
);

INSERT INTO hierarchical_structures (structure_id, name, description, file_f, file_g, file_h) VALUES
    (1, 'Projects Subordinate to Parts',
        'PART segment contains part#, name, description, qty-on-hand, qty-on-order; PROJECT child contains project#, name, description, qty-committed',
        'PART (part#, part_name, part_description, quantity_on_hand, quantity_on_order) -> PROJECT (project#, project_name, project_description, quantity_committed)',
        NULL, NULL),
    (2, 'Parts Subordinate to Projects',
        'PROJECT segment contains project#, name, description; PART child contains part#, name, description, qty-on-hand, qty-on-order',
        'PROJECT (project#, project_name, project_description) -> PART (part#, part_name, part_description, quantity_on_hand, quantity_on_order)',
        NULL, NULL),
    (3, 'Parts and Projects as Peers, Commitment Subordinate to Projects',
        'File F has PART segment, File G has PROJECT segment with PART child containing quantity committed',
        'PART (part#, part_name, part_description, quantity_on_hand, quantity_on_order)',
        'PROJECT (project#, project_name, project_description) -> PART (part#, quantity_committed)',
        NULL),
    (4, 'Parts and Projects as Peers, Commitment Subordinate to Parts',
        'File F has PART with PROJECT child, File G has PROJECT segment',
        'PART (part#, part_description, quantity_on_hand, quantity_on_order) -> PROJECT (project#, quantity_committed)',
        'PROJECT (project#, project_name, project_description)',
        NULL),
    (5, 'Parts, Projects, and Commitment as Peers',
        'Three separate files: PART, PROJECT, and COMMIT',
        'PART (part#, part_name, part_description, quantity_on_hand, quantity_on_order)',
        'PROJECT (project#, project_name, project_description)',
        'COMMIT (part#, project#, quantity_committed)');

-- ============================================================================
-- Relational concepts catalog
-- Documents the key concepts introduced in the paper
-- ============================================================================
CREATE TABLE relational_concepts (
    concept     TEXT PRIMARY KEY,
    section     TEXT NOT NULL,
    description TEXT NOT NULL
);

INSERT INTO relational_concepts (concept, section, description) VALUES
    ('relation',           '1.3', 'A set of n-tuples; each row is an n-tuple, all rows are distinct, column ordering is significant'),
    ('domain',             '1.3', 'The set of values represented at some instant; the active domain is values currently in the data bank'),
    ('primary_key',        '1.3', 'A domain (or combination) whose values uniquely identify each element (n-tuple) of the relation'),
    ('foreign_key',        '1.3', 'A domain of relation R whose elements are values of the primary key of some relation S'),
    ('normal_form',        '1.4', 'A relation whose domains are all simple (atomic) — no repeating groups or nested relations'),
    ('normalization',      '1.4', 'The process of eliminating nonsimple domains by expanding primary keys downward through the hierarchy'),
    ('permutation',        '2.1.1', 'Reordering columns of a relation; interchanging columns of a binary relation yields the converse'),
    ('projection',         '2.1.2', 'Selecting certain columns and removing duplicate rows from the result'),
    ('natural_join',       '2.1.3', 'Combining two relations on a common domain, preserving all information; always exists for joinable relations'),
    ('composition',        '2.1.4', 'Projection of natural join onto non-joining domains; R·S = π_13(R*S) for binary R, S'),
    ('restriction',        '2.1.5', 'A subset of relation R generated by matching against relation S on specified domains'),
    ('strong_redundancy',  '2.2.1', 'A relation possessing a projection derivable from other projections in the set'),
    ('weak_redundancy',    '2.2.2', 'A relation having a projection not derivable from others, but derivable from some join of projections'),
    ('consistency',        '2.3',   'The state where the named set of relations satisfies all time-independent constraints (Z) given values (V)'),
    ('data_independence',  '1.1',   'Independence of application programs from changes in data types and representation');
