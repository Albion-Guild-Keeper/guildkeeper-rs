tuo_progetto_workspace/
├── Cargo.toml          # <-- ROOT WORKSPACE TOML

├── apps/               # Directory per gli eseguibili principali (servizi/app)
│   ├── rest_api/
│   │   ├── Cargo.toml  # Dipende da core_lib, actix, shuttle, ecc.
│   │   └── src/
│   │       ├── main.rs       # Entry point Actix + Shuttle. Usa core_lib.
│   │       ├── routes/       # Specifico dell'API
│   │       ├── controllers/  # Specifico dell'API
│   │       └── errors_api.rs # (Opzionale) Errori specifici dell'API, può usare core_lib::Error
│   │
│   ├── site/
│   │   ├── Cargo.toml  # Dipende da core_lib (per tipi?), sycamore, trunk, ecc.
│   │   ├── index.html    # Necessario per Trunk
│   │   ├── Trunk.toml    # Configurazione di Trunk
│   │   └── src/
│   │       └── main.rs       # Entry point Sycamore. Usa core_lib (magari solo per tipi condivisi).
│   │       ├── pages/        # Specifico del frontend
│   │       ├── components/   # Specifico del frontend
│   │       └── app.rs        # Componente Root di Sycamore
│   │
│   └── discord_bot/
│       ├── Cargo.toml  # Dipende da core_lib, serenity, shuttle, ecc.
│       └── src/
│           ├── main.rs       # Entry point Serenity + Shuttle. Usa core_lib.
│           ├── commands/     # Specifico del bot
│           ├── handlers/     # Specifico del bot
│           └── errors_bot.rs # (Opzionale) Errori specifici del bot
│
├── tools/              # Directory per gli strumenti di supporto
│   └── project_cli/
│       ├── Cargo.toml  # Dipende da core_lib, clap (per parsing argomenti), ecc.
│       └── src/
│           └── main.rs       # Entry point della CLI. Usa core_lib per accedere a DB, logica.
│           ├── commands/     # Moduli per i sottocomandi (es. db, users)
│           │   ├── mod.rs
│           │   ├── db_cmd.rs
│           │   └── user_cmd.rs
│           └── args.rs       # Definizione argomenti con Clap
│
└── libs/               # Directory per le librerie condivise
    └── core_lib/       # La nostra libreria principale condivisa
        ├── Cargo.toml  # Contiene dipendenze comuni (sqlx, serde, thiserror, ecc.)
        └── src/
            ├── lib.rs        # Esporta i moduli pubblici
            ├── errors.rs     # Definizioni di errori condivisi (enum Error, Result<T>)
            ├── traits.rs     # Tratti custom condivisi
            ├── database.rs   # Logica DB condivisa (pool, funzioni di accesso, magari tratti repository)
            ├── models.rs     # Struct/enum condivise (es. User, Product) usate da API, Bot, CLI, forse Site
            └── utils.rs      # Altre utility condivise
