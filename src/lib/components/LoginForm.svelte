<script lang="ts">
    export let username: string;
    export let password: string;
    export let verifyCredentials: (username: string, password: string) => void;  // Adjusted function signature    export let gotoRegister: () => void;
    export let gotoRegister: () => void;
</script>

<form class="space-y-6" action="#" method="POST">

    <!-- Username -->
    <div>
        <label for="username" class="block text-sm font-medium leading-6 text-white">Username</label>
        <div class="mt-2">
            <input id="username" name="username" type="text" required bind:value={username}
                   autocomplete="off"
                   class="block w-full rounded-md border-0 bg-white/5 py-1.5 text-white shadow-sm ring-1 ring-inset ring-white/10 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6">
        </div>
    </div>

    <!-- Password -->
    <div>
        <div class="flex items-center justify-between">
            <label for="password" class="block text-sm font-medium leading-6 text-white">Password</label>
            <div class="text-sm">
                <a href="/" class="font-semibold text-indigo-400 hover:text-indigo-300">Forgot password?</a>
            </div>
        </div>
        <div class="mt-2">
            <input id="password" name="password" type="password" required bind:value={password}
                   autocomplete="off"
                   class="block w-full rounded-md border-0 bg-white/5 py-1.5 text-white shadow-sm ring-1 ring-inset ring-white/10 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6">
        </div>
    </div>

    <!-- Buttons -->
    <div>
        <!-- Sign In -->
        <button type="submit"
                class="flex w-full justify-center rounded-md bg-red-500 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-red-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
                on:click|preventDefault={() => verifyCredentials(username, password)}>
            Sign In
        </button>
        <!-- Register -->
        <button type="submit"
                class="flex w-full justify-center rounded-md bg-blue-500 px-3 py-1.5 mt-3 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-blue-700 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
                on:click|preventDefault={gotoRegister}>
            Register
        </button>
    </div>
</form>
