use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub done: bool,
}

// Fonction pour charger les tâches depuis un fichier JSON
pub fn load_tasks(path: &Path) -> Result<Vec<Task>> {
    if !path.exists() {
        // Si le fichier n'exites pas, on retourne un vecteur vide
        return Ok(vec![]);
    }
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;
    Ok(tasks)
}

// Fonction pour sauvegarder les tâches dans un fichier JSON
pub fn save_tasks(path: &Path, tasks: &[Task]) -> Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks)?;
    Ok(())
}

pub fn add_task(path: &Path, title: String) -> Result<Task> {
    let mut tasks = load_tasks(path)?; // Charge les tâches existantes
    let id = tasks.last().map(|t| t.id + 1).unwrap_or(1); // Génère un nouvel id (le dernier +1 ou 1 si vide)
    let task = Task {
        id,
        title,
        done: false,
    }; // Crée une nouvelle tâche non terminée
    tasks.push(task.clone()); // Ajoute la tâche à la liste
    save_tasks(path, &tasks)?; // Sauvegarde la liste mise à jour
    Ok(task) // Retourne la tâche ajoutée
}

pub fn mark_done(path: &Path, id: u64) -> Result<Option<Task>> {
    let mut tasks = load_tasks(path)?;
    let mut done_task = None;

    for t in tasks.iter_mut() {
        if t.id == id {
            t.done = true;
            done_task = Some(t.clone()); // On clone la tâche modifiée ici, pour la garder
            break; // On sort de la boucle
        }
    }
    if done_task.is_some() {
        save_tasks(path, &tasks)?; // On sauvegarde **après** la boucle, donc pas d'emprunts conflictuels
    }
    Ok(done_task) // On retourne la tâche modifiée ou None
}
