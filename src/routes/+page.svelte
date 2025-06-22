<script lang="ts">
  import { goto } from '$app/navigation';
  import { invoke } from "@tauri-apps/api/core";

  let username = '';
  let password = '';
  let loginError = '';
  let isLoading = false;

  async function handleLogin(event: Event) {
    event.preventDefault();
    loginError = '';
    isLoading = true;

    try {
      // Call Rust backend for login
      const isValid = await invoke<boolean>('login', { username, password });
      if (isValid) {
        goto('/pos');
      } else {
        loginError = "Invalid username or password.";
      }
    } catch (err) {
      loginError = "Login failed. Please try again!";
      console.log('Login failed:', err);
    }

    isLoading = false;
  }
</script>

<main class="container">
  <form class="login-form" on:submit={handleLogin}>
    <h1 class="login-title">POS System Login</h1>
    <div class="input-group">
      <label for="username">Username</label>
      <input
        id="username"
        type="text"
        placeholder="Enter your username..."
        bind:value={username}
        required
      />
    </div>

    <div class="input-group">
      <label for="password">Password</label>
      <input
        id="password"
        type="password"
        placeholder="Enter your password..."
        bind:value={password}
        required
      />
    </div>

    {#if loginError}
      <p class="error-message">{loginError}</p>
    {/if}

    <button type="submit" disabled={isLoading}>
      {#if isLoading}
        Logging in... 💅
      {:else}
        Login!
      {/if}
    </button>
  </form>
</main>

<style>

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6; /* This will be overridden by dark mode bg-image */

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  min-height: 90vh;
  overflow-y: hidden;
  padding-right: calc(67px + 2rem);
}


h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button { /* Keeping this style */
  cursor: pointer;
}

button:hover { /* Keeping this style */
  border-color: #396cd8;
}
button:active { /* Keeping this style */
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button { /* Keeping this style */
  outline: none;
}


@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-image: url("/bg1.jpg"); /* This is your beautiful login background! */
    background-repeat: no-repeat;
    background-position: center center;
    background-size: cover;
  }


  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98; /* Glassy input/button background in dark mode */
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
/* --- Original Styles End Here --- */



  .login-title {
    font-size: 2em; /* A bit larger for the main title */
    margin-bottom: 1.5rem; /* Space below the title */
  }

  .login-form {
    /* This creates the distinct login box with glassy effect */
    display: flex;
    flex-direction: column;
    gap: 1rem; /* Space between input groups and button */
    align-items: stretch; /* Stretch items to fill the form's width */
    max-width: 400px; /* Limit the width of the login form itself */
    width: 100%; /* Make it responsive within max-width */
    margin: 0 auto; /* Center the form horizontally within the container */
    padding: 2.5rem; /* Internal padding for the login box */
    border-radius: 40px; /* Slay-worthy rounded corners for the login box */
    background: rgba(44, 44, 44, 0.5); /* Glassy background */
    backdrop-filter: blur(10px) saturate(180%);
    -webkit-backdrop-filter: blur(10px) saturate(180%);
    border: 1px solid rgba(255, 255, 255, 0.15);
    box-shadow: 0 4px 30px rgba(0, 0, 0, 0.3);
    box-sizing: border-box; /* Include padding in width */
  }

  .input-group {
    width: 100%; /* Ensure it takes full width of the form */
    text-align: left; /* Align label and potentially input text left */
  }

  .input-group label {
    display: block; /* Make label take its own line */
    margin-bottom: 0.5rem; /* Space between label and input */
    font-size: 1rem;
    color: #ccc; /* Lighter color for labels */
    font-weight: 500;
  }

  /* Specific input styles for the login form to override defaults if needed */
  input { /* This targets ALL inputs, but your original input styles are already very good. */
    /* Just ensuring it takes full width and has correct padding/border-radius for login */
    width: 100%;
    border-radius: 40px; /* More rounded inputs for login vibe */
    box-sizing: border-box; /* Crucial for width calculation */
    /* Other styles inherited from your existing 'input' rule and dark mode */
  }

  input:focus {
    border-color: #ffffff;
  }

  /* Specific button styles for the login form to override defaults */
  button[type="submit"] { /* Targeting the submit button specifically for login look */
    /* Overrides the default button colors with your signature purple */
    background-color: #A020F0;
    color: #ffffff;
    box-shadow: 0 4px 15px rgba(160, 32, 240, 0.4); /* Stronger glow */
    border-color: transparent; /* Ensure no default border shows initially */
    border-radius: 40px; /* More rounded button for login vibe */
    padding: 0.8em 1.5em; /* Ensure generous padding */
    margin-top: 1rem;
  }

  button[type="submit"]:hover {
    background-color: #8a00d9;
    border-color: #a020f0;
    box-shadow: 0 4px 20px rgba(160, 32, 240, 0.6);
  }
  button[type="submit"]:active {
    background-color: #6a00a0;
  }

  button[type="submit"]:disabled {
    background-color: rgba(160, 32, 240, 0.4);
    cursor: not-allowed;
    box-shadow: none;
    border-color: transparent;
  }

  .error-message {
    color: #ff6347; /* Tomato red for errors, visually striking! */
    font-size: 0.9em;
    margin-top: -0.5rem; /* Pull it closer to the input */
    margin-bottom: 1rem;
    text-align: center; /* Center the error message */
  }
</style>
