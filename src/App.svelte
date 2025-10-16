<script lang="ts">
	import Dashboard from '$lib/components/dashboard/Dashboard.svelte';
	import LocalModels from '$lib/components/local-models/LocalModels.svelte';
	import Chat from '$lib/components/chat/Chat.svelte';
	import { Button } from '$lib/components/ui/button';
	import 'svelte-select/tailwind.css';

	let page: 'dashboard' | 'local-models' | 'chat' = $state('dashboard');
</script>

<div class="flex min-h-screen w-full flex-col bg-muted/40">
	<aside class="fixed inset-y-0 left-0 z-10 hidden w-60 flex-col border-r bg-background sm:flex">
		<nav class="flex flex-col gap-2 p-4">
			<Button variant={page === 'dashboard' ? 'secondary' : 'ghost'} onclick={() => (page = 'dashboard')}>
				Dashboard
			</Button>
			<Button variant={page === 'local-models' ? 'secondary' : 'ghost'} onclick={() => (page = 'local-models')}>
				Local Models
			</Button>
			<Button variant={page === 'chat' ? 'secondary' : 'ghost'} onclick={() => (page = 'chat')}>
				Chat
			</Button>
		</nav>
	</aside>
	<main class="flex flex-col sm:gap-4 sm:py-4 sm:pl-64 h-screen">
		{#if page === 'dashboard'}
			<Dashboard />
		{:else if page === 'local-models'}
			<LocalModels />
		{:else if page === 'chat'}
			<Chat />
		{/if}
	</main>
</div>