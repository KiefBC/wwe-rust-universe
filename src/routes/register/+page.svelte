<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";
    import {goto} from '$app/navigation';

    let username = '';
    let password = '';
    let errorMsg = '';
    let isError = false;

    /*
    This is for forcing the user to enter a username and password of certain length.
    This is called by the Register Button on the /Register page.
     */
    const verifyCredentials = async () => {
        console.log('Verifying credentials...');

        // Confirm the values are greater than 4
        if (username.length < 4 && password.length < 4) {
            errorMsg = `Username & Password must be at least 4 characters long.`;
            isError = true;
            return;
        } else if (username.length < 4) {
            errorMsg = `Username must be at least 4 characters long.`;
            isError = true;
            return;
        } else if (password.length < 4) {
            errorMsg = `Password must be at least 4 characters long.`;
            isError = true;
            return;
        } else {
            console.log(`Username: ${username}, Password: ${password}`);
            isError = false;

            try {
                // Send the credentials to the backend to register the user
                const registerUser = await invoke('register_user', {username: username, password: password});

                // If the user is registered, redirect to the login page
                if (registerUser) {
                    await goto('/');
                }
            } catch (error) {
                console.error(error);
                errorMsg = `User already exists. Please try again.`;
                isError = true;
            }
        }
    }

    /*
    This is used for Registering Users into the system.
    This is called by the verifyCredentials function.
    It then calls register_user in the backend.
     */
</script>

<div class="flex min-h-full flex-col justify-center px-6 py-3 lg:px-8">
    <div class="sm:mx-auto sm:w-full sm:max-w-sm">
        <img class="mx-auto h-32 w-auto" src="register_logo.png"
             alt="Your Company">
        <h2 class="mt-6 text-center text-2xl font-bold leading-9 tracking-tight text-white">SEX HAVERS</h2>
    </div>

    <!-- Form -->
    <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
        <form class="space-y-6" action="#" method="POST">

            <!-- Username -->
            <div>
                <label for="username" class="block text-sm font-medium leading-6 text-white">Username</label>
                <div class="mt-2">
                    <input id="username" name="username" type="text" required bind:value={username}
                           class="block w-full rounded-md border-0 bg-white/5 py-1.5 text-white shadow-sm ring-1 ring-inset ring-white/10 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6">
                </div>
            </div>

            <!-- Password -->
            <div>
                <div class="flex items-center justify-between">
                    <label for="password" class="block text-sm font-medium leading-6 text-white">Password</label>
                    <div class="text-sm">
                        <a href="#" class="font-semibold text-indigo-400 hover:text-indigo-300">Forgot password?</a>
                    </div>
                </div>
                <div class="mt-2">
                    <input id="password" name="password" type="password" required bind:value={password}
                           class="block w-full rounded-md border-0 bg-white/5 py-1.5 text-white shadow-sm ring-1 ring-inset ring-white/10 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6">
                </div>
            </div>

            <!-- Buttons -->
            <div>
                <!-- Register -->
                <button type="submit"
                        class="flex w-full justify-center rounded-md bg-blue-500 px-3 py-1.5 mt-3 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-blue-700 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
                        on:click|preventDefault={verifyCredentials}>
                    Register
                </button>
            </div>
        </form>

        {#if isError}
            <div class="rounded-md bg-yellow-50 p-4 mt-3">
                <div class="flex">
                    <div class="flex-shrink-0">
                        <svg class="h-5 w-5 text-yellow-400" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                            <path fill-rule="evenodd" d="M8.485 2.495c.673-1.167 2.357-1.167 3.03 0l6.28 10.875c.673 1.167-.17 2.625-1.516 2.625H3.72c-1.347 0-2.189-1.458-1.515-2.625L8.485 2.495zM10 5a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0v-3.5A.75.75 0 0110 5zm0 9a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd" />
                        </svg>
                    </div>
                    <div class="ml-3">
                        <h3 class="text-sm font-medium text-yellow-800">Attention needed</h3>
                        <div class="mt-2 text-sm text-yellow-700">
                            <p>{errorMsg}</p>
                        </div>
                    </div>
                </div>
            </div>

        {/if}

        <p class="mt-10 text-center text-sm text-gray-400">
            App built with Sveltekit and Tailwind
        </p>
    </div>
</div>

<style>
    @media (prefers-color-scheme: dark) {
        :root {
            color: #f6f6f6;
            background-color: #2f2f2f;
        }
    }
</style>

<slot/>