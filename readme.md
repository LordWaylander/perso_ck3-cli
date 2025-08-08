# générateur de personnage pour CK3

##installation : 
```
git clone ck3-cli
cd ck3-cli
git clone [ck3-core](https://github.com/LordWaylander/perso_ck3-core)
cargo build --release

```

```
Usage: ck3-player [OPTIONS]

Options:
  -e, --education <EDUCATION>  Possible values : [martialite, diplomatie, intrigue, intendance, erudition]
  -l, --level <LEVEL>          Possible values : [1, 2, 3, 4, 5]
  -a, --age <AGE>
  -h, --help                   Print help
  -V, --version                Print version

Si pas d'options -> random 100%
```


@todo : 
  - select age (calcul ?)
  - points max
  - personalité ?
  - affinage ?
  - save perso favori
