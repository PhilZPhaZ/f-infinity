# Introducing *f-infinity*

f-infinity est une nouvelle manière de représenter les nombres à virgule flottante. Grâce à la puissance de Rust, f-infinity est un outil puissant capable de représenter les nombres de manière exacte.

## Comment l'utiliser

Pour le moment, il n'existe pas de bibliothèque spécifique, mais vous pouvez l'utiliser via **main.rs**. Il existe actuellement trois types de nombres possibles :

- **BigNumber** : Une manière de stocker des très grands nombres.
- **SmallNumber** : Une manière de stocker des très petits nombres.
- **RationalNumber** : Une manière de stocker des fractions.

### Création de nombres

Chaque type de nombre peut être créé comme suit :

```rust
// Créer un SmallNumber
let small_number = SmallNumber::new("2.3456");

// Créer un BigNumber
let big_number = BigNumber::new("12345678901234567890");

// Créer un RationalNumber
let rational_number = RationalNumber::new("22/7");
```

Operations can be performed on these numbers:
  - BigNumber: Addition, Subtraction, Multiplication and Division
  - SmallNumber: Addition, Subtraction and Multiplication
  - RationalNumber: Addition only for the moment


## Contributing
You can help the project by improving performance and adding new traits and functions to the numbers.
