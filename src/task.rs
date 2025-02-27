#[derive(Debug, Clone)]
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
    pub fn add_child_task(&mut self, name: &str, estimate: i32) -> String {
        let child = Task::new(&self.get_child_id(), name, estimate);
        let id = child.id.clone();
        self.children.push(child);
        id
    }
}
