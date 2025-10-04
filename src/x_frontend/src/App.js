import { html, render } from 'lit-html';
import { x_backend } from 'declarations/x_backend';

class App {
  greeting = '';
  items = [];
  newItem = '';
  user = '';

  constructor() {
    this.user = localStorage.getItem('name') || '';
    if (this.user) {
      this.#loadItems();
    }
    this.#render();
  }

  #handleSubmit = async (e) => {
    e.preventDefault();
    let name = document.getElementById('name').value;
    this.greeting = await x_backend.greet(name);
    this.user = name;
    localStorage.setItem('name', name);
    await this.#loadItems();
    this.#render();
  };

  #handleAddItem = async (e) => {
    e.preventDefault();
    if (this.newItem.trim()) {
      await x_backend.addtolist(this.user, this.newItem.trim());
      this.newItem = '';
      await this.#loadItems();
      this.#render();
    }
  };

  #handleToggleState = async (id) => {
    await x_backend.changestate(this.user, id);
    await this.#loadItems();
    this.#render();
  };

  #handleRemoveItem = async (id) => {
    await x_backend.removetolist(this.user, id);
    await this.#loadItems();
    this.#render();
  };

  #handleRemoveAll = async () => {
    await x_backend.removeall(this.user);
    await this.#loadItems();
    this.#render();
  };

  #handleDeleteUser = async () => {
    await x_backend.deleteuser(this.user);
    this.user = '';
    this.items = [];
    localStorage.removeItem('name');
    this.greeting = '';
    this.#render();
  };

  #loadItems = async () => {
    this.items = await x_backend.showlist(this.user);
  };

  #render() {
    let body;
    if (!this.user) {
      body = html`
        <main>
          <form action="#">
            <label for="name">Enter your name: &nbsp;</label>
            <input id="name" alt="Name" type="text" />
            <button type="submit">Click Me!</button>
          </form>
          <section id="greeting">${this.greeting}</section>
        </main>
      `;
    } else {
      body = html`
        <main>
          <h1>Welcome, ${this.user}!</h1>
          <form @submit=${this.#handleAddItem}>
            <label for="newItem">Add new item: &nbsp;</label>
            <input id="newItem" .value=${this.newItem} @input=${(e) => this.newItem = e.target.value} alt="New Item" type="text" />
            <button type="submit">Add</button>
          </form>
          <ul>
            ${this.items.map(item => html`
              <li>
                <input type="checkbox" .checked=${item.state} @change=${() => this.#handleToggleState(item.id)} />
                <span style=${item.state ? 'text-decoration: line-through;' : ''}>${item.data}</span>
                <button @click=${() => this.#handleRemoveItem(item.id)}>Remove</button>
              </li>
            `)}
          </ul>
          <button @click=${this.#handleRemoveAll}>Remove All</button>
          <button @click=${this.#handleDeleteUser}>Delete User</button>
          <button @click=${() => { this.user = ''; localStorage.removeItem('name'); this.#render(); }}>Back</button>
        </main>
      `;
    }
    render(body, document.getElementById('root'));
    if (!this.user) {
      document
        .querySelector('form')
        .addEventListener('submit', this.#handleSubmit);
    }
  }
}

export default App;
