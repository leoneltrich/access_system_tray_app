<script lang="ts">
    import {onMount} from 'svelte';
    import {Plus} from 'lucide-svelte';
    import {goto} from '$app/navigation';

    import {servers} from '$lib/stores/servers';

    import {ServerService} from '$lib/services/servers';
    import {SettingsService} from "$lib/services/settings";

    import ServerCard from '$lib/components/dashboard/ServerCard.svelte';
    import {isAuthenticated} from "$lib/stores/auth";

    let pollInterval: any;

    onMount(() => {

        const setup = async () => {
            await SettingsService.load();
            await ServerService.load();
        }

        setup();

        return () => {
            if (pollInterval) clearInterval(pollInterval);
        }
    });

    $: handleServerPolling($isAuthenticated);

    function handleServerPolling(authenticated: boolean) {
        if (authenticated) {
            console.log("Authenticated state is true, syncing servers.");
            ServerService.syncAll();

            if (pollInterval) clearInterval(pollInterval);
            pollInterval = setInterval(() => ServerService.syncAll(), 5000);
        } else {
            if (pollInterval) {
                console.log("Authenticated state is false, stopping polling.");
                clearInterval(pollInterval);
                pollInterval = null;
            }
        }
    }

    function goAddServer() {
        goto('/add-server');
    }

</script>

<div class="view-content">
    <div class="view-header">
        <h2 class="view-title">Dashboard</h2>
    </div>

    <div class="view-body">
        {#if $servers.length === 0}
            <div class="empty-state">
                <p>No servers configured.</p>
                <span class="hint">Click the + button below to add one.</span>
            </div>
        {:else}
            <div class="grid-container">
                {#each $servers as server (server.id)}
                    <ServerCard
                            {server}
                            ondelete={() => ServerService.remove(server.id)}
                    />
                {/each}
            </div>
        {/if}
    </div>

    <div class="footer">
        <button class="add-server-btn" onclick={goAddServer}>
            <Plus size={18}/>
            <span>Add Server</span>
        </button>
    </div>
</div>

<style>
    .view-content {
        display: flex;
        flex-direction: column;
        height: 100%;
        position: relative;
    }

    .view-header {
        height: 2rem;
        display: flex;
        align-items: center;
        margin-bottom: 1.5rem;
        flex-shrink: 0;
    }

    .view-title {
        margin: 0;
        font-size: 1.25rem;
        font-weight: 600;
    }

    .view-body {
        flex: 1;
        overflow-y: auto;
        padding-right: 4px;
        scrollbar-width: none;
    }

    .view-body::-webkit-scrollbar {
        display: none;
    }

    .empty-state {
        height: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        color: #666;
    }

    .hint {
        font-size: 0.8rem;
        color: #444;
    }

    .grid-container {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
        gap: 12px;
        padding-bottom: 1rem;
    }

    .footer {
        margin-top: 1rem;
        flex-shrink: 0;
    }

    .add-server-btn {
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        background: #ffffff;
        color: #000000;
        border: none;
        padding: 10px;
        border-radius: 8px;
        font-weight: 600;
        font-size: 0.9rem;
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .add-server-btn:hover {
        background: #f0f0f0;
    }
</style>