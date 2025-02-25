pub struct Estimate {
    pub value: i32,
    pub name: String,
}

pub struct Task {
    pub name: String,
    pub estimate: i32,
    pub selected: bool,
    pub children: Vec<Task>,
}

pub struct EstimateApp {
    tasks: Vec<Task>,
}

impl EstimateApp {
    pub fn new() -> Self {
        Self {
            tasks: Self::get_example_tasks(),
        }
    }
    fn get_example_tasks() -> Vec<Task> {
        let mut tasks = Vec::new();
        for i in 0..10 {
            tasks.push(Self::get_example_task(i));
        }
        tasks
    }
    fn get_example_task(count: i32) -> Task {
        Task {
            children: Vec::new(),
            estimate: 16,
            selected: true,
            name: format!("Example task {}", count),
        }
    }
    pub fn get_tasks_mut(&mut self) -> &mut Vec<Task> {
        &mut self.tasks
    }
    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
}
