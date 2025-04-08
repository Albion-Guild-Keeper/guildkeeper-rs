// IN: apps/rest_api/src/state.rs
use std::sync::Arc;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;

// Importa la struct Settings specifica dell'API
use crate::config::Settings;

// Struct che contiene tutte le risorse condivise dall'applicazione API
#[derive(Clone)] // Necessario per Actix
pub struct AppState {
    // Connessione al Database SurrealDB
    pub db: Surreal<Any>,

    // Configurazione caricata (avvolta in Arc per efficienza)
    pub settings: Arc<Settings>,
    /*
    Il tipo Arc<Settings> viene utilizzato qui per permettere la condivisione sicura ed efficiente della struttura Settings tra diverse parti del programma, potenzialmente eseguite su thread differenti.

        Arc (Atomically Reference Counted): È un tipo di puntatore intelligente (smart pointer) in Rust. Permette a più "proprietari" di avere accesso agli stessi dati allocati sullo heap.
        Conteggio dei Riferimenti: Arc tiene traccia di quanti riferimenti attivi esistono per i dati che contiene. Ogni volta che si clona un Arc, il contatore interno viene incrementato. Quando un Arc viene distrutto (esce dallo scope), il contatore viene decrementato. I dati vengono deallocati solo quando il contatore raggiunge zero.
        Atomico: La parte "Atomically" significa che l'incremento e il decremento del contatore dei riferimenti sono operazioni atomiche. Questo garantisce che siano sicure da usare anche in contesti concorrenti (multi-threading), prevenendo race condition.
        Condivisione Immutabile: Arc fornisce accesso condiviso immutabile ai dati per impostazione predefinita. Se fosse necessaria la mutabilità condivisa tra thread, si userebbe Arc<Mutex<T>> o Arc<RwLock<T>>.
        Efficienza: Clonare un Arc è molto economico. Invece di copiare l'intera struttura Settings (che potrebbe essere grande), viene solo copiato il puntatore Arc e incrementato il contatore dei riferimenti.
        In sintesi, Arc<Settings> viene usato perché le settings devono probabilmente essere accessibili da più punti del codice (ad esempio, diversi gestori di richieste in un server web, thread di background, ecc.) in modo sicuro per i thread e senza dover duplicare l'intera configurazione ogni volta.
    */
    // Aggiungi qui altri campi se necessario:
    // pub redis_pool: Option<deadpool_redis::Pool>,
    // pub http_client: reqwest::Client,
}
