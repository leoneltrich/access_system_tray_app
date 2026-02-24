<script lang="ts">
    import {onMount} from 'svelte';
    import {CircleCheck} from 'lucide-svelte';
    import {isAuthenticated, authLoading, authError} from '$lib/stores/auth';
    import {AuthService} from '$lib/services/auth';
    import FormInput from '$lib/components/ui/FormInput.svelte';

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

<div class="view-content">
    <div class="view-header">
        <h2 class="view-title">Login</h2>
    </div>

    <div class="view-body">
        {#if !$isAuthenticated}
            <form class="login-form" onsubmit={handleLogin}>
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

                <div class="footer">
                    <button
                            type="submit"
                            class="action-btn primary"
                            disabled={$authLoading}
                    >
                        {#if $authLoading}Signing in...{:else}Sign In{/if}
                    </button>
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

            <div class="footer">
                <button class="action-btn secondary" onclick={handleLogout}>
                    Sign Out
                </button>
            </div>
        {/if}

    </div>
</div>

<style>
    /* --- LAYOUT --- */
    .view-content {
        display: flex;
        flex-direction: column;
        height: 100%;
    }

    .view-header {
        height: 2rem;
        display: flex;
        align-items: center;
        margin-bottom: 2rem;
        flex-shrink: 0;
    }

    .view-title {
        margin: 0;
        font-size: 1.25rem;
        font-weight: 600;
    }

    .view-body {
        flex: 1;
        display: flex;
        flex-direction: column;
    }

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

    /* --- FOOTER --- */
    .footer {
        margin-top: auto;
        padding-top: 1rem;
        flex-shrink: 0;
    }

    .action-btn {
        width: 100%;
        padding: 10px;
        border-radius: 8px;
        font-weight: 600;
        font-size: 0.9rem;
        cursor: pointer;
        border: none;
        transition: opacity 0.2s;
    }

    .action-btn:hover {
        opacity: 0.9;
    }

    .action-btn.primary {
        background: white;
        color: black;
    }

    .action-btn.secondary {
        background: #222;
        border: 1px solid #333;
        color: #ddd;
    }

    /* --- LOGGED IN ICONS --- */
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