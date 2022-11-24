use crux_core::*;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct TodoMVC {}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Todo {
    content: String,
    state: TodoState,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum TodoState {
    Active,
    Completed,
}

impl Default for TodoState {
    fn default() -> Self {
        TodoState::Active
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Model {
    pub todos: Vec<Todo>,
    pub filter: Option<TodoState>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ViewModel {
    pub todos: Vec<Todo>,
}

#[derive(Serialize, Deserialize)]
pub enum Msg {
    None,
    AddTodo(Todo),
    UpdateTodo(usize, Todo),
    Filter(TodoState),
}

impl App for TodoMVC {
    type Message = Msg;
    type Model = Model;
    type ViewModel = ViewModel;

    fn update(&self, msg: Msg, model: &mut Model) -> Vec<Command<Msg>> {
        match msg {
            Msg::None => {
                vec![]
            }
            Msg::AddTodo(todo) => {
                model.todos.push(todo);
                let bytes = serde_json::to_vec(&model).unwrap();
                vec![
                    key_value::write("state".to_string(), bytes, |_| Msg::None),
                    Command::render(),
                ]
            }
            Msg::UpdateTodo(i, todo) => {
                model.todos[i] = todo;
                let bytes = serde_json::to_vec(&model).unwrap();
                vec![
                    key_value::write("state".to_string(), bytes, |_| Msg::None),
                    Command::render(),
                ]
            }
            Msg::Filter(state) => {
                model.filter = Some(state);
                vec![Command::render()]
            }
        }
    }

    fn view(&self, model: &Model) -> ViewModel {
        let todos = match &model.filter {
            Some(state) => model
                .todos
                .clone()
                .into_iter()
                .filter(|t| t.state == *state)
                .collect(),
            None => model.todos.clone(),
        };

        ViewModel { todos }
    }
}
