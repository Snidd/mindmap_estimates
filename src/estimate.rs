pub struct Estimate {
    pub value: i32,
    pub name: String,
}

pub struct Task {
    pub id: String,
    pub name: String,
    pub estimate: i32,
    pub children: Vec<Task>,
}

impl Task {
    pub fn new(id: &str, name: &str, estimate: i32) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            estimate,
            children: Vec::new(),
        }
    }
    fn get_child_id(&self) -> String {
        //let id = format!("{}-{}", &self.id, self.children.len());
        let mut counter = self.children.len();
        loop {
            let candidate = format!("{}-{}", self.id, counter);
            if self.children.iter().all(|child| child.id != candidate) {
                return candidate;
            }
            counter += 1;
        }
    }
    pub fn add_child_task(&mut self, name: &str, estimate: i32) {
        let child = Task::new(&self.get_child_id(), name, estimate);
        self.children.push(child);
    }
}

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
    pub fn add_task(&mut self, name: &str) {
        self.tasks
            .push(Task::new(&format!("task-{}", self.tasks.len()), name, 0));
    }
}
