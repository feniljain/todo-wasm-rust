use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    tasks: Vec<Task>,
    input: String,
    editstr: String,
}

struct Task {
    task: String,
    edit: bool,
}

enum Message {
    AddTask,
    RemoveTask,
    UpdateInput(String),
    Edit(usize),
    UpdateTask(usize),
    UpdateEdit(String),
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            tasks: Vec::new(),
            input: String::new(),
            editstr: String::new(),
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::AddTask => {
                self.tasks.push(Task {
                    task: self.input.clone(),
                    edit: false,
                });
                self.input = String::new();
            }
            Message::RemoveTask => match self.tasks.pop() {
                _ => (),
            },
            Message::UpdateInput(s) => {
                self.input = s;
            }
            Message::Edit(i) => {
                self.tasks[i].edit = true;
                self.editstr = self.tasks[i].task.clone();
            }
            Message::UpdateTask(i) => {
                self.tasks[i].task = self.editstr.clone();
                self.tasks[i].edit = false;
            }
            Message::UpdateEdit(s) => self.editstr = s,
        }

        true
    }

    fn view(&self) -> Html {
        let task = |(i, t): (usize, &Task)| {
            if !t.edit {
                html! {
                    <div>
                        <p>
                            {&t.task}
                            <button onclick=self.link.callback(move |_| Message::Edit(i))>{"Edit"}</button>
                        </p>
                    </div>
                }
            } else {
                html! {
                    <div>
                        <p>
                            <input
                                type="text",
                                placeholer = "Updated task",
                                value = &self.editstr,
                                oninput=self.link.callback(move |e: InputData| Message::UpdateEdit(e.value)),
                            />
                            <button onclick=self.link.callback(move |_| Message::UpdateTask(i))>{"Done"}</button>
                        </p>
                    </div>
                }
            }
        };

        html! {
            <div>
                <p>{"TODO WASM RUST"}</p>
                <input
                    type="text",
                    placeholer = "So what should we complete today?",
                    value = &self.input,
                    oninput=self.link.callback(|e: InputData| Message::UpdateInput(e.value)),
                />
                <button onclick=self.link.callback(|_| Message::AddTask)>{"Add"}</button>
                <button onclick=self.link.callback(|_| Message::RemoveTask)>{"Remove"}</button>
                { for self.tasks.iter().enumerate().map(task) }
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
