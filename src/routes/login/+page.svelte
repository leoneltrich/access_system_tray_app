<script lang="ts">
    import {onMount} from 'svelte';
    import {CircleCheck, Plus} from 'lucide-svelte';
    import {isAuthenticated, authLoading, authError} from '$lib/stores/auth';
    import {AuthService} from '$lib/services/auth';
    import FormInput from '$lib/components/ui/FormInput.svelte';
    import PageView from "$lib/components/ui/PageView.svelte";

    let username = $state("");
    let password = $state("");
    let isTouched = $state(false);

    let showError = $derived(!!$authError && !isTouched);

    onMount(() => {
    });

    function handleLogin(event?: Event) {
        if (event) event.preventDefault();
        if (!username || !password) return;

        isTouched = false;
        AuthService.login(username, password);
    }

    function handleLogout() {
        username = "";
        password = "";
        AuthService.logout();
    }

    function handleInput() {
        isTouched = true;
    }
</script>

<PageView title="Login">
    <div class="view-body">
        {#if !$isAuthenticated}
            <form id="login-form" class="login-form" onsubmit={handleLogin}>
                <div class="content-wrapper">
                    <p class="subtitle">Please sign in to continue.</p>

                    <div class="inputs-stack">
                        <FormInput
                                id="username"
                                label="Username"
                                hideLabel={true}
                                placeholder="Username"
                                bind:value={username}
                                oninput={handleInput}
                                autocomplete="username"
                                disabled={$authLoading}
                                error={showError ? true : undefined}
                                required
                        />

                        <FormInput
                                id="password"
                                type="password"
                                label="Password"
                                hideLabel={true}
                                placeholder="Password"
                                bind:value={password}
                                oninput={handleInput}
                                autocomplete="current-password"
                                disabled={$authLoading}
                                error={showError ? $authError : undefined}
                                required
                        />
                    </div>
                </div>
            </form>

        {:else}
            <div class="content-wrapper centered">
                <div class="success-icon">
                    <CircleCheck size={48} color="#10b981"/>
                </div>
                <h3>Authenticated</h3>
                <p>You are logged in securely.</p>
            </div>
        {/if}
    </div>

    {#snippet footer()}
        {#if !$isAuthenticated}
            <button
                    form="login-form"
                    type="submit"
                    class="primary-action-btn"
                    disabled={$authLoading}
            >
                {#if $authLoading}Signing in...{:else}Sign In{/if}
            </button>
        {:else}
            <button class="primary-action-btn secondary-action-btn" onclick={handleLogout}>
                Sign Out
            </button>
        {/if}
    {/snippet}
</PageView>

<style>

    .login-form {
        display: flex;
        flex-direction: column;
        flex: 1;
    }

    .content-wrapper {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .content-wrapper.centered {
        align-items: center;
        justify-content: center;
        height: 100%;
        text-align: center;
    }

    .inputs-stack {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .subtitle {
        margin: 0;
        color: #888;
        font-size: 0.9rem;
    }

    .success-icon {
        background: rgba(16, 185, 129, 0.1);
        padding: 1rem;
        border-radius: 50%;
        margin-bottom: 1rem;
        display: flex;
    }

    h3 {
        margin: 0 0 0.5rem 0;
        font-weight: 600;
        font-size: 1.1rem;
    }

    p {
        color: #888;
        margin: 0;
        font-size: 0.9rem;
    }
</style>