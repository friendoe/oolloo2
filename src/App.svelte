<script lang="ts">
	import Dashboard from '$lib/components/dashboard/Dashboard.svelte';
	import LocalModels from '$lib/components/local-models/LocalModels.svelte';
	import { Button } from '$lib/components/ui/button';

	let page: 'dashboard' | 'local-models' = $state('dashboard');
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
		</nav>
	</aside>
	<main class="flex flex-col sm:gap-4 sm:py-4 sm:pl-64">
		{#if page === 'dashboard'}
			<Dashboard />
		{:else if page === 'local-models'}
			<LocalModels />
		{/if}
	</main>
</div>