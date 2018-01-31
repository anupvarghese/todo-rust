use std::io;

struct Todo {
  id: i16,
  title: String,
  completed: bool,
  deleted: bool,
}

fn add_todo(todos: &mut Vec<Todo>, title: &str) {
  let new_id = todos.len() as i16 + 1;
  todos.push(Todo {
    id: new_id,
    title: title.to_string(),
    completed: false,
    deleted: false,
  });
}


fn show_todos(todos: &Vec<Todo>) {
  println!("\n\nTodo List:\n---------------", );
  for todo in todos {
    if !todo.deleted {
      let done = if todo.completed { "✔" } else { " " };
      println!("[{}] {} {}", done, todo.id, todo.title);
    }
  }
}

fn remove_todos(todos: &mut Vec<Todo>, todo_id: i16) {
  if let Some(todo) = todos.iter_mut().find(|todo| todo.id == todo_id) {
    todo.deleted = true;
  }
}

fn mark_done(todos: &mut Vec<Todo>, todo_id: i16) {
  if let Some(todo) = todos.iter_mut().find(|todo| todo.id == todo_id) {
    todo.completed = true;
  }
}

fn invalid_command(command: &str) {
  println!("Invalid command: {}", command);
}

fn main() {
  let mut todos: Vec<Todo> = Vec::new();
  show_todos(&todos);

  loop {
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("failed to read line");

    let command_parts: Vec<&str> = command.split_whitespace().collect();

    match command_parts.len() {
      0 => invalid_command(&command),
      1 => match command_parts[0] {
        "list" => show_todos(&todos),
        _ => invalid_command(&command),
      },
      _ => {
        match command_parts[0] {
          "add" => add_todo(&mut todos, &command_parts[1..].join(" ")),
          "remove" => if let Ok(num) = command_parts[1].parse::<i16>() {
            remove_todos(&mut todos, num);
          },
          "done" => if let Ok(num) = command_parts[1].parse::<i16>() {
            mark_done(&mut todos, num);
          }
          _ => invalid_command(&command)
        }
      }
    }
    show_todos(&todos);
  }
}
