## Rust Quicksort Implementation

This project provides an implementation of the Quicksort algorithm in Rust.



## Installation

To run the Quicksort algorithm on a sample array, follow these steps:

1. **Install Rust if you dont have:**

Unix/Linux:
```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
Windows:

    Download the Rust installer.

    Run the installer and follow the on-screen instructions.

    Restart your computer to apply the changes.

2. **Clone the repository:**
  ```bash
    git clone https://github.com/JuanMarceloT/QuickSort.git
    cd quicksort
  ```

3. **Run the code** 
```bash

  cargo run

```
 This will run the Quicksort algorithm on a sample array and print the sorted array.f i you want you can implement some way to get the numbers from a file or just put your numbers int arr variable.


## How to Test

To run tests for the Quicksort implementation, use the following command:

```bash

cargo test -- --nocapture
```

The --nocapture flag is used to show any print statements in the test output. and will run 20 times with 100k random values and will print the first 50 elements from the array unsorted and sorted for each time 

