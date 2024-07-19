<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";
    import {goto} from '$app/navigation';
</script>

<div class="grid grid-cols-2 gap-4 overflow-hidden rounded-lg bg-gray-500 shadow">
    <div class="p-6 m-4 bg-blue-900 shadow rounded-lg">
        <div class="flex items-center justify-center flex-col">
            <img src="https://placehold.co/200" alt="Placeholder" class="rounded-lg mx-auto">
            <p class="mt-2 mb-2">Don't have an account?</p>
            <button type="button" class="bg-blue-500 text-white rounded-lg p-2" on:click={() => goto('/register')}>Register</button>
            <button type="button" class="bg-blue-500 text-white rounded-lg p-2" on:click={() => goto('/login')}>Login</button>
        </div>
    </div>
    <div class="p-6 m-4 bg-blue-900 shadow rounded-lg">
        THIS IS SOME WORDS
    </div>
    <div class="p-6 m-4 bg-blue-900 shadow rounded-lg">
        THIS IS SOME WORDS
    </div>
    <div class="p-6 m-4 bg-blue-900 shadow rounded-lg">
        THIS IS SOME WORDS
    </div>
</div>

