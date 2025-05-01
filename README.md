# 🧠 SHINA — Smart Heuristic Interpreter for Nefarious Activities

**Shina** is a lightweight, fast, and extensible rule-based detection engine designed for log analysis, anomaly hunting, and embedded detection in SIEM pipelines.
It turns simple, human-readable expressions into structured logic trees and evaluates them against raw and parsed logs.

![banner](./.github/banner.png)

## 💡 What is SHINA?
SHINA stands for:
> **S**mart **H**euristic **I**nterpreter for **N**efarious **A**ctivities

It is a DSL-powered engine that lets you write detection logic like this:
```
path == "/vulns" && user-agent CONTAINS "fox"
```
…and automatically transforms it into structured conditions and evaluates them efficiently.

## 🔮 Understanding the engine

Shina is developed for a specific purpose: to detect nefarious activities in logs. But semi parsed logs. Work need to be done BEFORE using Shina. For example, on your serveur web if you got the line of log as below:
```
2025-05-01 - 200 GET [704e9ad3-78f1-44ff-a4ef-5ae0fcade0ef] /admin
```
You need to parse it as an HashMap. For example:
```json
{
  "date": "2025-05-01",
  "status": 200,
  "method": "GET",
  "id": "704e9ad3-78f1-44ff-a4ef-5ae0fcade0ef",
  "path": "/admin"
}
```
Shina will next be able to evaluate multiple conditions, based on the `parsed log` and the `raw log`.
Like this, in deep context you can have condition and link betwen the `parsed log` and the `raw log`.


## 🔖 Langage

### Logical connectors

| Connector | Name | Description |
| --------- | ---- | ----------- |
| `&&` | AND | Logical AND, allows you to combine (X) AND (Y) |
| `\|\|` | OR | Logical OR, allows you to combine (X) OR (Y) |

### Comparison operators

Basics of comparison operators, will be able to compare the value of the left operand with the right operand.
These operators are used to compare the parsed log values.
 
| Operator | Name | Description |
| -------- | ---- | ----------- |
| `==` | Equal | Checks if the left operand is equal to the right operand |
| `!=` | Not equal | Checks if the left operand is not equal to the right operand |
| `>>` | Greater than | Checks if the left operand is greater than the right operand |
| `<<` | Less than | Checks if the left operand is less than the right operand |
| `>=` | Greater than or equal to | Checks if the left operand is greater than or equal to the right operand |
| `<=` | Less than or equal to | Checks if the left operand is less than or equal to the right operand |

Advanced comparison operators:

| Operator | Name | Description |
| -------- | ---- | ----------- |
| `CONTAINS` | Contains | Checks if the left operand contains the right operand |
| `NOTCONTAINS` | Not contains | Checks if the left operand does not contain the right operand |


Raw data comparison operators:

| Operator | Name | Description |
| -------- | ---- | ----------- |
| `RCONTAINS` | Contains | Check if the raw string contains the data |
| `RNOTCONTAINS` | Not contains | Check if the raw string does not contain the data |



## 🚀 Getting started
You can test shina with this repository, so test it with the command:
```bash
git clone https://github.com/mlab-sh/Shina
cd Shina
cargo run
```
And you may have this to check if everything is ok:
```bash
                      
  _____ _   _         
 |   __| |_|_|___ ___ 
 |__   |   | |   | .'|
 |_____|_|_|_|_|_|__,|
                 
->> Shina - Rule Engine
-----------------------------------
Parsed Rule: "Fox detector"
Parsed Rule: Or(Equals("path", "/vulns"), And(Contains("user-agent", "fox"), Equals("method", "GET")))
Rule weight: 10
Matched: true
-----------------------------------
```
