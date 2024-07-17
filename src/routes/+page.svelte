<script>
    import {onMount} from 'svelte';
    import {goto} from '$app/navigation';

    // Redirect to the profile page
    // This is for development purposes
    onMount(() => {
        goto('/profile');
    });
</script>