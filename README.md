# Why use this package?

This package allows you to write and read txt file with one line of code, rather then having to wast time and write 50-100 lines of code!

# How do I read files?

There are two ways to read a file. One reads a file line by line(read), one reads it as a string (read_one).

Line by line version

```
fn main {
    let data = txt_writer::ReadData {}
            .read("src/data.txt".to_string())
            .expect("failed when reading");
        for x in data {
            println!("{}", x);
        }
}
```

## All in one version

```
fn main() {
 let data = txt_writer::ReadData {}
        .read_one("src/data.txt".to_string())
        .expect("failed when reading");
    println!("{}", data);
}

```

# How do I write data?
