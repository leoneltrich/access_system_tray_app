<script lang="ts">
    import { onMount } from 'svelte';
    import { CheckCircle2, AlertCircle } from 'lucide-svelte';

    // Import our new Auth Logic
    import {
        isAuthenticated,
        authLoading,
        authError,
        login,
        logout,
        initAuth
    } from '$lib/stores/auth';

    let username = "";
    let password = "";

    // Initialize auth check when view opens
    onMount(() => {
        initAuth();
    });

    function handleLogin() {
        if(!username || !password) return;
        login(username, password);
    }

    function handleLogout() {
        // Clear local inputs on logout for security
        username = "";
        password = "";
        logout();
    }
</script>

<div class="view-content">
    <div class="view-header">
        <h2 class="view-title">Profile</h2>
    </div>

    <div class="view-body">

        {#if !$isAuthenticated}
            <div class="login-form">
                <p class="subtitle">Please sign in to continue.</p>

                {#if $authError}
                    <div class="error-banner">
                        <AlertCircle size={16} />
                        <span>{$authError}</span>
                    </div>
                {/if}

                <div class="input-group">
                    <input
                            type="text"
                            placeholder="Username"
                            bind:value={username}
                            spellcheck="false"
                            disabled={$authLoading}
                    />
                    <input
                            type="password"
                            placeholder="Password"
                            bind:value={password}
                            disabled={$authLoading}
                            on:keydown={(e) => e.key === 'Enter' && handleLogin()}
                    />
                </div>

                <button
                        class="action-btn primary"
                        on:click={handleLogin}
                        disabled={$authLoading}
                >
                    {#if $authLoading}
                        Signing in...
                    {:else}
                        Sign In
                    {/if}
                </button>
            </div>

        {:else}
            <div class="logged-in-state">
                <div class="success-icon">
                    <CheckCircle2 size={48} color="#10b981" />
                </div>
                <h3>Authenticated</h3>
                <p>You are logged in securely.</p>

                <button class="action-btn secondary" on:click={handleLogout}>
                    Sign Out
                </button>
            </div>
        {/if}

    </div>
</div>

<style>
    /* --- SHARED LAYOUT --- */
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
    }

    .view-title {
        margin: 0;
        font-size: 1.25rem;
        font-weight: 600;
        line-height: 1;
    }

    .view-body {
        flex: 1;
        display: flex;
        flex-direction: column;
        justify-content: center;
    }

    /* --- LOGIN FORM --- */
    .login-form {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .subtitle {
        margin: 0;
        color: #888;
        font-size: 0.9rem;
    }

    .error-banner {
        background: rgba(239, 68, 68, 0.1);
        border: 1px solid rgba(239, 68, 68, 0.2);
        color: #ef4444;
        padding: 10px;
        border-radius: 8px;
        font-size: 0.85rem;
        display: flex;
        align-items: center;
        gap: 8px;
        animation: fadeIn 0.3s ease;
    }

    .input-group {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
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

    input:focus {
        border-color: #555;
        background: #222;
    }

    input:disabled {
        opacity: 0.5;
        cursor: wait;
    }

    /* --- LOGGED IN STATE --- */
    .logged-in-state {
        display: flex;
        flex-direction: column;
        align-items: center;
        text-align: center;
        animation: fadeIn 0.4s ease;
    }

    .success-icon {
        background: rgba(16, 185, 129, 0.1);
        padding: 1rem;
        border-radius: 50%;
        margin-bottom: 1rem;
        display: flex;
    }

    .logged-in-state h3 {
        margin: 0 0 0.5rem 0;
        font-weight: 600;
        font-size: 1.1rem;
    }

    .logged-in-state p {
        color: #888;
        margin: 0 0 2rem 0;
        font-size: 0.9rem;
    }

    @keyframes fadeIn {
        from { opacity: 0; transform: scale(0.95); }
        to { opacity: 1; transform: scale(1); }
    }

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
    .action-btn:disabled { opacity: 0.7; cursor: wait; }

    .action-btn.primary {
        background: white;
        color: black;
    }

    .action-btn.secondary {
        background: #222;
        border: 1px solid #333;
        color: #ddd;
    }
</style>