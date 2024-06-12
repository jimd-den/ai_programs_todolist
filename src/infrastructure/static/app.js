document.addEventListener('DOMContentLoaded', function() {
    const form = document.getElementById('todo-form');
    const newTodoInput = document.getElementById('new-todo');
    const todoList = document.getElementById('todo-list');

    form.addEventListener('submit', async function(event) {
        event.preventDefault();
        const title = newTodoInput.value.trim();
        if (title) {
            const todo = await createTodo(title);
            addTodoToList(todo);
            newTodoInput.value = '';
        }
    });

    todoList.addEventListener('click', async function(event) {
        if (event.target.classList.contains('delete')) {
            const id = event.target.dataset.id;
            await deleteTodo(id);
            event.target.parentElement.remove();
        } else if (event.target.classList.contains('toggle')) {
            const id = event.target.dataset.id;
            const todo = await getTodoById(id);
            todo.completed = !todo.completed;
            await updateTodo(todo);
            event.target.parentElement.classList.toggle('completed');
        }
    });

    async function fetchTodos() {
        const response = await fetch('/todos');
        if (!response.ok) {
            console.error('Failed to fetch todos:', response.statusText);
            return;
        }
        const todos = await response.json();
        todos.forEach(addTodoToList);
    }

    async function createTodo(title) {
        const response = await fetch('/todos', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(title)
        });
        if (!response.ok) {
            console.error('Failed to create todo:', response.statusText);
            return;
        }
        return await response.json();
    }

    async function getTodoById(id) {
        const response = await fetch(`/todos/${id}`);
        if (!response.ok) {
            console.error('Failed to fetch todo:', response.statusText);
            return;
        }
        return await response.json();
    }

    async function updateTodo(todo) {
        const response = await fetch(`/todos/${todo.id}`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(todo)
        });
        if (!response.ok) {
            console.error('Failed to update todo:', response.statusText);
            return;
        }
    }

    async function deleteTodo(id) {
        const response = await fetch(`/todos/${id}`, { method: 'DELETE' });
        if (!response.ok) {
            console.error('Failed to delete todo:', response.statusText);
            return;
        }
    }

    function addTodoToList(todo) {
        const li = document.createElement('li');
        li.className = todo.completed ? 'completed' : '';
        li.innerHTML = `
            ${todo.title}
            <button class="toggle" data-id="${todo.id}">${todo.completed ? 'Undo' : 'Complete'}</button>
            <button class="delete" data-id="${todo.id}">Delete</button>
        `;
        todoList.appendChild(li);
    }

    fetchTodos();
});
