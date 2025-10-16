<script lang="ts">
	import { onMount } from 'svelte';
	import ModelCard from './ModelCard.svelte';
	import { Input } from '$lib/components/ui/input';
	import { Search } from 'svelte-lucide';

	type Model = {
		name: string;
		model: string;
		modified_at: string;
		size: number;
		digest: string;
		details: {
			parent_model: string;
			format: string;
			family: string;
			families: string[] | null;
			parameter_size: string;
			quantization_level: string;
		};
	};

	let models: Model[] = $state([]);
	let loading = $state(true);
	let searchQuery = $state('');

	async function fetchModels() {
		loading = true;
		try {
			const response = await fetch('http://localhost:11434/api/tags');
			if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
			const data = await response.json();
			models = data.models || [];
		} catch (error) {
			console.error('Error fetching models:', error);
			// In a real app, you'd show a toast or a more user-friendly error message
			alert('Error fetching models. Please make sure Ollama is running.');
		} finally {
			loading = false;
		}
	}

	onMount(() => {
		fetchModels();
	});

	function handleModelDeleted(deletedModelName: string) {
		models = models.filter((model) => model.name !== deletedModelName);
	}

	const filteredModels = $derived(
		models.filter((model) => model.name.toLowerCase().includes(searchQuery.toLowerCase()))
	);
</script>

<div class="p-6 space-y-6">
	<div class="space-y-2">
		<h1 class="text-3xl font-bold">Local Models</h1>
		<p class="text-muted-foreground">Manage your locally installed Ollama models.</p>
	</div>

	<div class="relative">
		<Search class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
		<Input bind:value={searchQuery} placeholder="Search models..." class="pl-10" />
	</div>

	{#if loading}
		<div class="flex justify-center py-8">
			<p>Loading...</p>
			<!-- You can replace this with a spinner component if you have one -->
		</div>
	{:else if filteredModels.length === 0}
		<div class="text-center py-8">
			<h3 class="text-lg font-semibold">
				{#if searchQuery}
					No models found matching your search
				{:else}
					No models installed
				{/if}
			</h3>
			<p class="text-muted-foreground mt-1">
				{#if !searchQuery}
					Pull a model from the "Online Models" page to get started.
				{/if}
			</p>
		</div>
	{:else}
		<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
			{#each filteredModels as model (model.digest)}
				<ModelCard {model} onDelete={handleModelDeleted} />
			{/each}
		</div>
	{/if}
</div>