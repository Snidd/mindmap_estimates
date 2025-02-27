use crate::Task;

#[derive(Debug, Clone, Default)]
pub struct EstimateApp {
    pub tasks: Vec<Task>,
}

impl EstimateApp {
    pub fn new() -> Self {
        Self {
            tasks: Self::get_example_tasks(),
        }
    }
    fn get_example_tasks() -> Vec<Task> {
        let mut tasks = Vec::new();
        for i in 0..7 {
            tasks.push(Self::get_example_task(i));
        }
        tasks
    }
    fn get_example_task(count: i32) -> Task {
        if count == 0 || count == 3 {
            Task {
                children: vec![
                    Task::new(
                        format!("task-{}-1", count).as_str(),
                        format!("Example Task {} 1", count).as_str(),
                        4,
                    ),
                    Task::new(
                        format!("task-{}-2", count).as_str(),
                        format!("Example Task {} 2", count).as_str(),
                        4,
                    ),
                    Task::new(
                        format!("task-{}-3", count).as_str(),
                        format!("Example Task {} 3", count).as_str(),
                        4,
                    ),
                ],
                estimate: 16,
                id: format!("task-{}", count),
                name: format!("Example task {}", count),
            }
        } else {
            Task {
                children: Vec::new(),
                estimate: 16,
                id: format!("task-{}", count),
                name: format!("Example task {}", count),
            }
        }
    }
    pub fn get_tasks_mut(&mut self) -> &mut Vec<Task> {
        &mut self.tasks
    }
    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
    pub fn add_task(&mut self, name: &str) -> String {
        let task = Task::new(&format!("task-{}", self.tasks.len()), name, 0);
        let id = task.id.clone();
        self.tasks.push(task);
        id
    }

    /// Searches the tasks tree for a task matching the given `id` and returns a reference if found.
    pub fn find_task(&self, id: &str) -> Option<&Task> {
        for task in &self.tasks {
            if task.id == id {
                return Some(task);
            }
            if let Some(found) = Self::find_task_recursive(task, id) {
                return Some(found);
            }
        }
        None
    }

    /// Searches the tasks tree for a task matching the given `id` and returns a reference if found.
    pub fn find_mut_task(&mut self, id: &str) -> Option<&mut Task> {
        for task in &mut self.tasks {
            if task.id == id {
                return Some(task);
            }
            if let Some(found) = Self::find_mut_task_recursive(task, id) {
                return Some(found);
            }
        }
        None
    }

    /// Helper function that recursively searches the children of `task` for a matching task.
    fn find_mut_task_recursive<'a>(task: &'a mut Task, id: &str) -> Option<&'a mut Task> {
        for child in &mut task.children {
            if child.id == id {
                return Some(child);
            }
            if let Some(found) = Self::find_mut_task_recursive(child, id) {
                return Some(found);
            }
        }
        None
    }

    /// Helper function that recursively searches the children of `task` for a matching task.
    fn find_task_recursive<'a>(task: &'a Task, id: &str) -> Option<&'a Task> {
        for child in &task.children {
            if child.id == id {
                return Some(child);
            }
            if let Some(found) = Self::find_task_recursive(child, id) {
                return Some(found);
            }
        }
        None
    }

    /// Returns the next task ID in the flattened tasks tree.
    /// If `current_id` is provided, the next task in pre-order is returned (cycling back to the start).
    /// If `current_id` is None or not found, returns the first task ID if available.
    pub fn next_task_id(&self, current_id: Option<&str>) -> Option<String> {
        let flat_tasks = self.flatten_tasks();
        if flat_tasks.is_empty() {
            return None;
        }
        if let Some(id) = current_id {
            if let Some(index) = flat_tasks.iter().position(|task| task.id == id) {
                let next_index = (index + 1) % flat_tasks.len();
                return Some(flat_tasks[next_index].id.clone());
            }
        }
        // If no current_id is provided or it wasn't found, return the first task's ID.
        Some(flat_tasks[0].id.clone())
    }

    /// Returns the next task ID in the flattened tasks tree.
    /// If `current_id` is provided, the next task in pre-order is returned (cycling back to the start).
    /// If `current_id` is None or not found, returns the first task ID if available.
    pub fn previous_task_id(&self, current_id: Option<&str>) -> Option<String> {
        let flat_tasks = self.flatten_tasks();
        if flat_tasks.is_empty() {
            return None;
        }
        if let Some(id) = current_id {
            if let Some(index) = flat_tasks.iter().position(|task| task.id == id) {
                if index == 0 {
                    return Some(flat_tasks[flat_tasks.len() - 1].id.clone());
                }
                let next_index = index - 1;
                return Some(flat_tasks[next_index].id.clone());
            }
        }
        // If no current_id is provided or it wasn't found, return the first task's ID.
        Some(flat_tasks[0].id.clone())
    }

    /// Flattens the tasks tree into a pre-order vector of task references.
    fn flatten_tasks(&self) -> Vec<&Task> {
        let mut flat = Vec::new();
        for task in &self.tasks {
            Self::flatten_task_recursive(task, &mut flat);
        }
        flat
    }

    /// Recursively pushes `task` and its children into the provided vector.
    fn flatten_task_recursive<'a>(task: &'a Task, flat: &mut Vec<&'a Task>) {
        flat.push(task);
        for child in &task.children {
            Self::flatten_task_recursive(child, flat);
        }
    }
}
