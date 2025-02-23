use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};

use eframe::egui;

#[derive(Debug, Clone)]
struct Task {
    id: i32,
    title: String,
    description: String,
    completed: bool,
}

struct TaskManager {
    tasks: Vec<Task>,
    task_id: String,
    task_title: String,
    task_description: String,
}

impl Default for TaskManager {
    fn default() -> Self {
        Self {
            tasks: load_from_file(),
            task_id: String::new(),
            task_title: String::new(),
            task_description: String::new(),
        }
    }
}

impl eframe::App for TaskManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Task Manager");

            // Поля для ввода новой задачи
            ui.horizontal(|ui| {
                ui.label("ID: ");
                ui.text_edit_singleline(&mut self.task_id);

                ui.label("Title: ");
                ui.text_edit_singleline(&mut self.task_title);

                ui.label("Description: ");
                ui.text_edit_singleline(&mut self.task_description);

                if ui.button("Add Task").clicked() {
                    if let Ok(id) = self.task_id.trim().parse::<i32>() {
                        if check_unique_id(&mut self.tasks, id) {
                            self.tasks.push(Task {
                                id,
                                title: self.task_title.clone(),
                                description: self.task_description.clone(),
                                completed: false,
                            });
                            save_to_file(&self.tasks);
                            self.task_id.clear();
                            self.task_title.clear();
                            self.task_description.clear();
                        } else {
                            ui.colored_label(egui::Color32::RED, "Invalid ID!");
                        }
                    } else {
                        ui.colored_label(egui::Color32::YELLOW, "Error in ID");
                    }
                }
            });

            ui.separator();

            // Соберём ID задач для удаления
            let mut tasks_to_remove = Vec::new();

            // Вывод списка задач
            ui.heading("Task List:");

            egui::ScrollArea::vertical().show(ui, |ui| {
                for task in &mut self.tasks {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut task.completed, "");
                        ui.label(format!(
                            "{}: {} - {} [{}]",
                            task.id,
                            task.title,
                            task.description,
                            if task.completed { "Completed" } else { "Pending" }
                        ));

                        // Если нажата кнопка "Delete", добавляем ID задачи в список на удаление
                        if ui.button("Delete").clicked() {
                            tasks_to_remove.push(task.id);
                        }
                    });
                }
            });

            // Удалим задачи после завершения цикла
            if !tasks_to_remove.is_empty() {
                self.tasks.retain(|task| !tasks_to_remove.contains(&task.id));
                save_to_file(&self.tasks);
            }
        });
    }
}

fn load_from_file() -> Vec<Task> {
    let mut tasks = Vec::new();
    let file = OpenOptions::new().read(true).open("tasks.txt");

    if let Ok(file) = file {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let singleTask: Vec<&str> = line.split(',').collect();
                if singleTask.len() == 4 {
                    let id = singleTask[0].parse().unwrap();
                    let title = singleTask[1].to_string();
                    let description = singleTask[2].to_string();
                    let completed = singleTask[3].parse().unwrap_or(false);

                    tasks.push(Task {
                        id,
                        title,
                        description,
                        completed,
                    });
                }
            }
        }
        println!("Tasks loaded from file!")
    }
    tasks
}

fn save_to_file(tasks: &Vec<Task>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open("tasks.txt")
        .expect("Could not open tasks.txt");

    for task in tasks {
        let line = format!(
            "{},{},{},{}\n",
            task.id, task.title, task.description, task.completed
        );
        file.write_all(line.as_bytes()).unwrap();
    }
    println!("Tasks saved to file!");
}

fn show_tasks(tasks: &mut Vec<Task>) {
    for task in tasks {
        println!(
            "{},{},{},{}",
            task.id, task.title, task.description, task.completed
        );
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    let mut id = String::new();
    let mut title = String::new();
    let mut description = String::new();
    let mut completed = false;

    println!("\nAdding task:");

    println!("Enter task ID:");
    io::stdin().read_line(&mut id);
    let id: i32 = match id.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error in adding ID");
            return;
        }
    };

    if !check_unique_id(tasks, id) {
        return;
    }

    println!("Enter title:");
    io::stdin().read_line(&mut title);

    println!("Enter description:");
    io::stdin().read_line(&mut description);

    tasks.push(Task {
        id,
        title: title.trim().to_string(),
        description: description.trim().to_string(),
        completed,
    });

    println!("Task added!");

    save_to_file(&tasks);
}

fn check_unique_id(tasks: &mut Vec<Task>, id: i32) -> bool {
    for task in tasks {
        if task.id == id {
            println!("This task ID is already taken!");
            return false;
        }
    }
    true
}

fn delete_task(tasks: &mut Vec<Task>) {
    let mut id = String::new();

    println!("\nRemoving task:");

    println!("Enter task ID:");
    io::stdin().read_line(&mut id);

    let id: i32 = match id.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error in reading ID");
            return;
        }
    };

    if let Some(i) = tasks.iter().position(|task| task.id == id) {
        tasks.remove(i);
        println!("Task {} removed!", id);
    } else {
        println!("Task {} not found!", id);
    }

    save_to_file(&tasks);
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Task Manager",
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default().with_inner_size((600.0, 400.0)),
            ..Default::default()
        },
        Box::new(|_cc| Box::<TaskManager>::default()),
    )
}
