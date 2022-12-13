struct TodoModel {
    title: String,
    description: String,
}

#[tauri::command]
pub async fn create_todo() -> String {
    TodoModel.objects.create(title: "Hello", description: "World").await
}

#[tauri::command]
pub async fn gtt_all_todos() -> String {
    TodoModel.objects.all().await
}