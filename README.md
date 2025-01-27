# act2txt
Tool to convert Adobe Photoshop ACT palette files to Paint.NET TXT format. 
This was something I had already coded in C#, and I used this as an excuse 
to write some code in Rust.

You can install it using the command:

```
cargo install act2txt
```

# usage
```
act2txt [-f] [--all] inputfile outputfile
```

 Use `-f` option to force overwriting the output file if it exists

Example:

```
act2txt somepalette.act someplaette.txt
```

# license
MIT License
