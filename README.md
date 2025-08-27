# oxydu

fast disk usage analyzer. created with the purpose of learning rust from the basics without dependencies
doing the heavy lifting for me.

## roadmap

- [x] scan the current workdir and list the files present
- [x] get each file's logical & physical size
- [x] print a pretty, formatted output
- [x] show the size in appropriate units of magnitude (GiB, MiB, B) 
- [ ] dynamically change the width of columns based on terminal width
- [ ] add cli args for the target directory and human/machine readable sizes
- [ ] add a cli arg for depth and scan directories recursively until a depth is reached
