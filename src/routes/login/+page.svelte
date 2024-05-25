<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";

    let username = "";
    let password = "";
    let loginResponse = "";

    // These variables are used to change the color of the input fields based on the user input.
    $: usernameInput = username ? 'is-success' : 'is-danger';
    $: passwordInput = password ? 'is-success' : 'is-danger';
    $: loginResponseOutput = loginResponse === "Please enter a Username & Password." ? 'notification is-danger' : '';
    $: loginResponseOutput = loginResponse === "Username & Password Validated." ? 'notification is-success' : loginResponseOutput;

    /*
    This function is called when the form is submitted.
     */
    const login_validation = async () => {
        if (validate_login(username, password)) {
            loginResponse = await invoke("login_validation", { username: username, password: password });
        } else {
            loginResponse = "Please enter a Username & Password.";
        }
    };

    /*
    This function checks if the user inputted a valid username ad password.
     */
    const validate_login = (username: string, password: string): boolean => {
        return !(username === "" || password === "");
    };

    const changeView = () => {
        currentView.set("register");
    };
</script>

<div class="container px-5 has-text-centered">
    <p class="mb-5 has-text-danger">Please login to continue</p>
    <!-- Login Form -->
    <div class="field is-horizontal">
        <div class="field-body">
            <!-- user name input -->
            <div class="field">
                <p class="control has-icons-left has-icons-right">
                    <input class="input {usernameInput}" type="text" bind:value={username} placeholder="Username"/>
                    <span class="icon is-small is-left">
                        <FontAwesomeIcon icon={faUser}/>
                    </span>
                    <span class="icon is-small is-right">
                        <FontAwesomeIcon icon={faCheck}/>
                    </span>
                </p>
                {#if username === ""}
                    <p class="help is-danger">This username is EMPTY.</p>
                {:else if username.length > 3}
                    <p class="help is-success">This username is valid.</p>
                {:else}
                    <p class="help is-danger">This username is too short.</p>
                {/if}

            </div>

            <!-- password input -->
            <div class="field">
                <p class="control has-icons-left has-icons-right">
                    <input class="input {passwordInput}" type="text" bind:value={password} placeholder="Password"/>
                    <span class="icon is-small is-left">
                        <FontAwesomeIcon icon={faUser}/>
                    </span>
                    <span class="icon is-small is-right">
                        <FontAwesomeIcon icon={faCheck}/>
                    </span>
                </p>
                {#if password === ""}
                    <p class="help is-danger">This password is EMPTY.</p>
                {:else if password.length > 3}
                    <p class="help is-success">This password is valid.</p>
                {:else}
                    <p class="help is-danger">This password is too short.</p>
                {/if}
            </div>
        </div>
    </div>

    <!-- Submit button -->
    <div class="field">
        <div class="field-label">
            <!-- Left empty for spacing -->
        </div>
        <div class="field-body">
            <div class="field">
                <div class="control">
                    <!-- Login Button -->
                    <button class="button is-danger is-outlined" on:click={login_validation}>
                        Login
                    </button>
                    <!-- Register Button -->
                    <button class="ml-2 button is-info is-outlined" on:click={changeView}>
                        Register
                    </button>
                </div>
            </div>
        </div>
    </div>

    <!-- Display the greeting message -->
    <p id="login-response" class="{loginResponseOutput}">{loginResponse}</p>
</div>