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
            showDialog(todo);
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

    async function updateTodoStart(id, startTime) {
        const response = await fetch(`/todos/${id}/start`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ start_time: startTime })
        });
        if (!response.ok) {
            console.error('Failed to start todo:', response.statusText);
            return;
        }
    }

    async function updateTodoComplete(id, completeTime) {
        const response = await fetch(`/todos/${id}/complete`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ complete_time: completeTime })
        });
        if (!response.ok) {
            console.error('Failed to complete todo:', response.statusText);
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

    function showDialog(todo) {
        const dialog = document.createElement('div');
        dialog.className = 'dialog';
        dialog.innerHTML = `
            <p>Do you want to start or complete this todo?</p>
            <button class="start">Start</button>
            <button class="complete">Complete</button>
            <button class="cancel">Cancel</button>
        `;
        document.body.appendChild(dialog);

        dialog.querySelector('.start').addEventListener('click', async () => {
            const startTime = new Date().toISOString();
            await updateTodoStart(todo.id, startTime);
            dialog.remove();
        });

        dialog.querySelector('.complete').addEventListener('click', async () => {
            const completeTime = new Date().toISOString();
            await updateTodoComplete(todo.id, completeTime);
            dialog.remove();
        });

        dialog.querySelector('.cancel').addEventListener('click', () => {
            dialog.remove();
        });
    }

    fetchTodos();
});