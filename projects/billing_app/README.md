# Billing App

CLI app to track bills / expenses (CRUD)

Goals is to practice Rust concepts:

- Enums, Option, Result, match, iterators, etc
- Ownership / Borrowing / Mutability

The original implementation only needed very basic features, to practice more things I implemented field validations, more descriptive errors, proper encapsulation of the Bill struct and the BillStore, and the reducer pattern to dispatch actions, so my solution is very different than the original as a result

## Original Task Description

Project 1: Interactive bill manager

Create a command line bills/expenses manager that runs
interactively. This mini project brings together many of
the concepts learn thus far into a single application.

The user stories/requirements are split into stages.
Fully implement each stage as a complete working program
before making changes for the next stage. Leverage the
compiler by using `cargo check --bin p1` when changing
between stages to help identify adjustments that need
to be made.

### User stories

#### Stage 1

- I want to add bills, including the name and amount owed.
- I want to view existing bills.

#### Stage 2

- I want to remove bills.

#### Stage 3

- I want to edit existing bills.
- I want to go back if I change my mind.

### Tips

- Use the loop keyword to create an interactive menu.
- Each menu choice should be it's own function, so you can work on the
  the functionality for that menu in isolation.
- A vector is the easiest way to store the bills at stage 1, but a
  hashmap will be easier to work with at stages 2 and 3.
