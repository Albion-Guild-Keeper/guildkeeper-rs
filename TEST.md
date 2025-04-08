tuo_progetto_workspace/
├── .cargo/                  # (Opzionale) Configurazione di Cargo (es. .cargo/config.toml)
├── .github/                 # (Opzionale) Workflow CI/CD (GitHub Actions)
├── .vscode/                 # (Opzionale) Impostazioni specifiche per VS Code
├── Cargo.toml               # Workspace root: definisce membri, resolver, profili, [workspace.dependencies]
├── README.md                # Descrizione, Setup, Come Avviare/Testare/Deployare OGNI componente
├── .gitignore               # FONDAMENTALE: include /target, .env, out/, pkg/, *.log, etc.
├── .env.example             # File di esempio per .env (SENZA segreti reali)
# --- File di Configurazione Condivisi (NON SEGRETI) ---
├── config/
│   ├── default.toml         # Valori base comuni
│   ├── development.toml     # Override/Aggiunte per DEV
│   └── production.toml      # Override/Aggiunte per PROD
# --- Applicazioni Deployabili ---
├── apps/
│   ├── rest_api/            # <<< API REST (Actix-web + Shuttle) >>>
│   │   ├── Cargo.toml       # Deps: actix-web, serde, config, core_lib, shuttle-actix-web, etc.
│   │   └── src/
│   │       ├── main.rs          # Entry point: Carica config, setup stato (DB pool), avvia Actix App
│   │       ├── config.rs        # Define `ApiSettings`, carica config specifica API
│   │       ├── errors.rs        # `ApiError`, impl ResponseError, impl From<CoreError>, etc.
│   │       ├── middleware/      # (Opzionale) Middleware Actix (auth, logging)
│   │       │   ├── mod.rs
│   │       │   └── auth.rs
│   │       ├── features/        # <<< Organizzazione per Feature/Vertical Slice >>>
│   │       │   ├── mod.rs       # Registra i moduli feature per il routing in main.rs
│   │       │   ├── health/      # Feature: Health Check
│   │       │   │   ├── mod.rs       # Definisce routing (`register_routes`) e riesporta handler
│   │       │   │   └── handler.rs   # Handler per /health
│   │       │   ├── users/       # Feature: Gestione Utenti
│   │       │   │   ├── mod.rs       # Definisce routing (`register_routes`)
│   │       │   │   ├── handler.rs   # Handlers (riceve request, chiama service, ritorna response)
│   │       │   │   ├── service.rs   # Logica di business per utenti (usa core_lib::database)
│   │       │   │   └── dto.rs       # DTOs (Request/Response) specifici per utenti
│   │       │   └── products/    # Feature: Gestione Prodotti
│   │       │       ├── mod.rs
│   │       │       ├── handler.rs
│   │       │       ├── service.rs
│   │       │       └── dto.rs
│   │       └── state.rs         # (Opzionale) Struct per lo stato condiviso dell'app (AppState)
│   │                            #             se diventa complesso (contiene DbPool, config, etc.)
│   │
│   ├── discord_bot/         # <<< Discord Bot (Serenity + Shuttle) >>>
│   │   ├── Cargo.toml       # Deps: serenity, config, core_lib, shuttle-serenity, etc.
│   │   └── src/
│   │       ├── main.rs          # Entry point: Carica config, setup client Serenity, registra hooks/commands
│   │       ├── config.rs        # Define `BotSettings`, carica config specifica Bot
│   │       ├── errors.rs        # `BotError`, gestione errori specifica Bot
│   │       ├── features/        # <<< Organizzazione per Feature >>>
│   │       │   ├── mod.rs       # Raggruppa e registra feature (comandi/eventi) in main.rs
│   │       │   ├── moderation/  # Feature: Moderazione
│   │       │   │   ├── mod.rs
│   │       │   │   ├── commands.rs  # Comandi slash/prefix per moderazione
│   │       │   │   ├── events.rs    # Gestione eventi Discord relativi a moderazione
│   │       │   │   └── service.rs   # Logica di business per moderazione
│   │       │   └── fun_commands/ # Feature: Comandi Divertenti
│   │       │       ├── mod.rs
│   │       │       └── commands.rs
│   │       ├── hooks.rs         # (Opzionale) Serenity Hooks (es. before, after command)
│   │       └── handler.rs       # (Alternativa a features/events) Implementazione `serenity::EventHandler`
│   │
│   └── website/             # <<< Frontend (Sycamore + Trunk) >>>
│       ├── Cargo.toml       # Deps: sycamore, trunk, wasm-bindgen, reqwasm, etc. (NO core_lib pesante)
│       ├── index.html       # Punto di ingresso HTML per Trunk
│       ├── Trunk.toml       # Configurazione di Trunk (es. proxy per API in dev)
│       ├── static/          # Asset statici (CSS, immagini)
│       └── src/
│           ├── main.rs          # Entry point WASM, monta App Sycamore
│           ├── app.rs           # Componente Root dell'App Sycamore
│           ├── router.rs        # Gestione routing lato client
│           ├── components/      # Componenti UI riutilizzabili
│           │   ├── mod.rs
│           │   └── button.rs
│           ├── pages/           # Componenti che rappresentano pagine intere
│           │   ├── mod.rs
│           │   └── home_page.rs
│           ├── services/        # Moduli per interagire con l'API REST backend
│           │   ├── mod.rs
│           │   └── api_client.rs  # Funzioni per fetch (usa reqwasm)
│           └── types.rs         # Definizioni tipi/struct usate nel frontend (spesso speculari ai DTO API)
# --- Librerie Condivise Interne ---
├── libs/
│   └── core_lib/            # <<< Libreria Condivisa Core >>>
│       ├── Cargo.toml       # Deps: serde, sqlx, thiserror, etc. (MA NON framework web/bot!)
│       └── src/
│           ├── lib.rs           # Esporta i moduli pubblici
│           ├── config_models.rs # Struct condivise per config (es. DatabaseSettings)
│           ├── errors.rs        # `CoreError` enum e `Result<T, CoreError>` condiviso
│           ├── models/          # Modelli Dati del Dominio/Database
│           │   ├── mod.rs
│           │   └── user.rs      # Struct User (può derivare sqlx::FromRow)
│           │   └── product.rs
│           ├── persistence/     # Interazione con il Database (o altra persistenza)
│           │   ├── mod.rs       # Esporta pool creation e repositories
│           │   ├── db.rs        # Funzione per creare/gestire il pool (es. `create_pool`)
│           │   ├── user_repo.rs # Funzioni/Trait per operazioni DB sugli utenti
│           │   └── product_repo.rs# Funzioni/Trait per operazioni DB sui prodotti
│           └── utils/           # Utility veramente generiche (usare con cautela!)
│               ├── mod.rs
│               └── validation.rs # Esempio: Regole di validazione condivise
# --- Strumenti di Sviluppo/Supporto ---
└── tools/
    └── project_cli/         # <<< CLI per Gestione Progetto >>>
        ├── Cargo.toml       # Deps: clap, config, core_lib, etc.
        └── src/
            ├── main.rs          # Entry point: Carica config, parse args (Clap), dispatch command
            ├── config.rs        # Define `CliSettings`, carica config specifica CLI
            ├── errors.rs        # `CliError`, gestione errori specifica CLI
            ├── args.rs          # Definizione argomenti e sottocomandi con Clap derive
            └── commands/        # Moduli per l'implementazione di ogni sottocomando
                ├── mod.rs       # Esporta i moduli dei comandi
                ├── db.rs        # Logica per sottocomandi `db` (es. migrate, seed)
                └── users.rs     # Logica per sottocomandi `users` (es. list, grant-role)