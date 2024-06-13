# Rust Time Tracker
![Testing](https://github.com/ejendret/rust-time-tracker/actions/workflows/rust.yml/badge.svg)

### Goal
The goal for this project is to produce a command line tool in Rust that enables users to track time spent on various projects and tasks within said projects

### Services
User should be able to create, view, update and delete projects
User should be able to create, view, update and delete discrete tasks within a project

### Implementation
Data Model
Task
- Title
- Sessions
  - Start time
  - End time
  - Duration
- Total duration

Project
- Title
- Tasks
- Total duration
