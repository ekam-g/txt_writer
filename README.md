# Why use this package?

This package allows you to write and read txt file with one line of code, rather then having to waste time and write 10-50 lines of code!

# How do I read files?

There are two ways to read a file. One reads a file line by line(read), one reads it as a string (read_one).

## Line by line version

```
fn main() {
    let data = txt_writer::ReadData {}
            .read("path to your txt".to_string())
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
        .read_one("path to your txt".to_string())
        .expect("failed when reading");
    println!("{}", data);
}

```

# How do I write data?

There are 2 ways to write data. If you want to create a txt OR overwrite data use this.

## use this one if you want to pass a String value

```
fn main() {
    txt_writer::WriteData {}
        .drop_replace(
            "what you want to write to txt".to_string(),
            "path to your txt".to_string(),
        )
        .expect("failed when writing");
}
```

use this one if you want to pass a &String value

```
fn main() {
    txt_writer::WriteData {}
            .replace(
                &"what you want to write to txt".to_string(),
                "path to your txt".to_string(),
            )
            .expect("failed when writing");
}

```

## Use this one if you want to add data to an existing txt.

NOTE THIS FUNCTION WILL RETURN AN ERROR IF THE TXT DOES NOT EXIST

use this one if you want to pass a String value

```
fn main(){
    txt_writer::WriteData {}
            .drop_add(
                "what you want to write to txt".to_string(),
                "path to your txt".to_string(),
            )
            .expect("failed when writing");
}

```

use this one if you want to pass a &String value

```
fn main() {
    txt_writer::WriteData {}
            .add(
                &"what you want to write to txt".to_string(),
                "path to your txt".to_string(),
            )
            .expect("failed when writing");
}

```

# Example 

```
fn main() {
    txt_writer::WriteData {}
        .drop_replace(
            "what you want to write to txt".to_string(),
            "src/data.txt".to_string(),
        )
        .expect("failed when writing");

    txt_writer::WriteData {}
        .replace(
            &"what you want to write to txt".to_string(),
            "src/data.txt".to_string(),
        )
        .expect("failed when writing");

    txt_writer::WriteData {}
        .drop_add(
            "what you want to write to txt".to_string(),
            "src/data.txt".to_string(),
        )
        .expect("failed when writing");

    txt_writer::WriteData {}
        .add(
            &"what you want to write to txt".to_string(),
            "src/data.txt".to_string(),
        )
        .expect("failed when writing");

    let data = txt_writer::ReadData {}
        .read("src/data.txt".to_string())
        .expect("failed when reading");
    for x in data {
        println!("{}", x);
    }

    let data = txt_writer::ReadData {}
        .read_one("src/data.txt".to_string())
        .expect("failed when reading");
    println!("{}", data);
}
```