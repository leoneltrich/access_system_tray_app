<script lang="ts">
    import { CircleCheck } from 'lucide-svelte';
    import {
        isAuthenticated,
        authLoading,
        authError,
        login,
        logout
    } from '$lib/stores/auth';

    let username = $state("");
    let password = $state("");

    // "Dirty" state to clear errors when user starts typing
    let isTouched = $state(false);

    function handleLogin(event?: Event) {
        if (event) event.preventDefault();
        if (!username || !password) return;

        isTouched = false;
        login(username, password);
    }

    function handleLogout() {
        username = "";
        password = "";
        logout();
    }

    function handleInput() {
        isTouched = true;
    }
</script>

<div class="view-content">
    <div class="view-header">
        <h2 class="view-title">Profile</h2>
    </div>

    <div class="view-body">

        {#if !$isAuthenticated}
            <form class="login-form" onsubmit={handleLogin}>
                <div class="content-wrapper">
                    <p class="subtitle">Please sign in to continue.</p>

                    <div class="input-group">
                        <label for="username" class="sr-only">Username</label>
                        <input
                                id="username"
                                name="username"
                                type="text"
                                placeholder="Username"
                                bind:value={username}
                                oninput={handleInput}
                                spellcheck="false"
                                autocomplete="username"
                                disabled={$authLoading}
                                class:input-error={$authError && !isTouched}
                                required
                        />

                        <label for="password" class="sr-only">Password</label>
                        <input
                                id="password"
                                name="password"
                                type="password"
                                placeholder="Password"
                                bind:value={password}
                                oninput={handleInput}
                                autocomplete="current-password"
                                disabled={$authLoading}
                                class:input-error={$authError && !isTouched}
                                required
                        />

                        {#if $authError && !isTouched}
                            <p class="error-msg" role="alert">{$authError}</p>
                        {/if}
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
                    <CircleCheck size={48} color="#10b981" />
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
    .view-content { display: flex; flex-direction: column; height: 100%; }
    .view-header { height: 2rem; display: flex; align-items: center; margin-bottom: 2rem; flex-shrink: 0; }
    .view-title { margin: 0; font-size: 1.25rem; font-weight: 600; }

    .view-body { flex: 1; display: flex; flex-direction: column; }
    .login-form { display: flex; flex-direction: column; flex: 1; }

    .content-wrapper { display: flex; flex-direction: column; gap: 1.5rem; }
    .content-wrapper.centered { align-items: center; justify-content: center; height: 100%; text-align: center; }

    .footer { margin-top: auto; padding-top: 1rem; flex-shrink: 0; }

    /* --- TEXT & INPUTS --- */
    .subtitle { margin: 0; color: #888; font-size: 0.9rem; }
    .input-group { display: flex; flex-direction: column; gap: 0.75rem; }

    .sr-only {
        position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px;
        overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border-width: 0;
    }

    input {
        background: #1a1a1a;
        border: 1px solid #333;
        color: white;
        padding: 12px;
        border-radius: 8px;
        font-size: 0.95rem;
        outline: none;
        transition: all 0.2s;
    }
    input:focus { border-color: #555; background: #222; }
    input:disabled { opacity: 0.5; }
    input.input-error { border-color: #ef4444; color: #ef4444; }

    .error-msg {
        color: #ef4444;
        font-size: 0.8rem;
        margin: -0.25rem 0 0 2px;
        animation: fadeIn 0.3s ease;
    }

    /* --- LOGGED IN STATE --- */
    .success-icon {
        background: rgba(16, 185, 129, 0.1);
        padding: 1rem;
        border-radius: 50%;
        margin-bottom: 1rem;
        display: flex;
    }
    h3 { margin: 0 0 0.5rem 0; font-weight: 600; font-size: 1.1rem; }
    p { color: #888; margin: 0; font-size: 0.9rem; }

    /* --- BUTTONS --- */
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
    .action-btn:hover { opacity: 0.9; }
    .action-btn.primary { background: white; color: black; }
    .action-btn.secondary { background: #222; border: 1px solid #333; color: #ddd; }

    @keyframes fadeIn { from { opacity: 0; transform: translateY(-5px); } to { opacity: 1; transform: translateY(0); } }
</style>