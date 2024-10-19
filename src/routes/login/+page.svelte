<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { goto } from '$app/navigation';

    import LoginForm from '$lib/components/LoginForm.svelte';
    import ErrorAlert from '$lib/components/ErrorAlert.svelte';
    import Logo from '$lib/components/Logo.svelte';

    let errorMsg = '';  // Use only errorMsg to handle errors

    const verifyCredentials = async (username: string, password: string) => {
        console.log('Verifying credentials...');
        console.log(`Username: "${username}", Length: ${username.length}`); // Log username and length
        console.log(`Password: "${password}", Length: ${password.length}`); // Log password and length

        // Validate username and password length separately
        if (username.length < 4) {
            errorMsg = `Username must be at least 4 characters long.`;
            return;
        }

        if (password.length < 4) {
            errorMsg = `Password must be at least 4 characters long.`;
            return;
        }

        // Reset errorMsg before proceeding
        errorMsg = '';

        try {
            // Send the credentials to the backend to be verified
            const response = await invoke('verify_credentials', { susername: username, spassword: password });
            if (response) {
                console.log('Credentials verified successfully.');
                await goto('/profile');
            } else {
                console.error('Credentials verification failed.');
                errorMsg = `Invalid credentials. Please try again.`;
            }
        } catch (e) {
            console.error(e);
            errorMsg = `An error occurred while verifying credentials.`;
        }
    }

    const gotoRegister = () => {
        goto('/register');
    }
</script>

<div class="flex min-h-full flex-col justify-center">
    <Logo src="logo.png" />
    <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
        <LoginForm {verifyCredentials} {gotoRegister} />
        <ErrorAlert {errorMsg} />  <!-- Pass only errorMsg -->
        <p class="mt-10 text-center text-sm text-gray-400">App built with Sveltekit and Tailwind</p>
    </div>
</div>
