<script lang="ts">
	import { onMount } from 'svelte';
	import MetricCard from './MetricCard.svelte';
	import RunningModelRow from './RunningModelRow.svelte';
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Alert, AlertDescription, AlertTitle } from '$lib/components/ui/alert';
	import { Terminal, Zap } from 'svelte-lucide';
	import { Separator } from '$lib/components/ui/separator';
	import { Badge } from '$lib/components/ui/badge';

	type Model = {
		name: string;
		digest: string;
		size: number;
		size_vram: number;
		details: {
			quantization_level: string;
		};
	};

	type TagsData = {
		models: Model[];
	};

	type PsData = {
		models: Model[];
	};

	let runningModels: Model[] = $state([]);
	let stats = $state({ totalModels: 0, totalDiskUsage: 0 });
	let loading = $state(true);
	let error: string | null = $state(null);

	onMount(() => {
		const fetchData = async () => {
			loading = true;
			error = null;

			try {
				const [psResponse, tagsResponse] = await Promise.all([
					fetch('http://localhost:11434/api/ps'),
					fetch('http://localhost:11434/api/tags')
				]);

				if (!psResponse.ok || !tagsResponse.ok) {
					throw new Error('Failed to fetch data from Ollama');
				}

				const [psData, tagsData]: [PsData, TagsData] = await Promise.all([
					psResponse.json(),
					tagsResponse.json()
				]);

				runningModels = psData.models || [];
				const totalSize = (tagsData.models || []).reduce(
					(acc: number, model: Model) => acc + (model.size || 0),
					0
				);
				stats = {
					totalModels: (tagsData.models || []).length,
					totalDiskUsage: totalSize
				};
			} catch (err: any) {
				console.error('Error fetching data:', err);
				error = err.message;
			} finally {
				loading = false;
			}
		};

		fetchData();

		const interval = setInterval(fetchData, 30000);
		return () => clearInterval(interval);
	});

	const totalVram = $derived(
		runningModels.reduce((acc, model) => acc + (model.size_vram || 0), 0)
	);
</script>

<div class="p-6 space-y-6">
	<!-- Header -->
	<div class="space-y-2">
		<h1 class="text-3xl font-bold">Dashboard</h1>
		<p class="text-muted-foreground">Monitor your Ollama models and system performance</p>
	</div>

	{#if error}
		<Alert variant="destructive" class="mb-3">
			<Terminal class="h-4 w-4" />
			<AlertTitle>Error</AlertTitle>
			<AlertDescription>
				{error} - Please make sure Ollama is running on localhost:11434
			</AlertDescription>
		</Alert>
	{/if}

	<!-- Metrics Grid -->
	<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
		<MetricCard
			title="Total Models"
			value={stats.totalModels}
			description={`${stats.totalModels} models installed`}
		/>
		<MetricCard
			title="Running Models"
			value={runningModels.length}
			description={`${runningModels.length} active in memory`}
		/>
		<MetricCard
			title="Memory Usage"
			value={`${(totalVram / 1e9).toFixed(1)} GB`}
			description="VRAM occupied by running models"
		/>
		<MetricCard
			title="Disk Usage"
			value={`${(stats.totalDiskUsage / 1e9).toFixed(1)} GB`}
			description="Total space used by all models"
		/>
	</div>

	<!-- Running Models Section -->
	<div class="space-y-4">
		<div class="flex items-center gap-2">
			<Zap class="h-5 w-5" />
			<h2 class="text-xl font-semibold">Running Models</h2>
			<Badge variant="secondary">{runningModels.length}</Badge>
		</div>
		<Card>
			<CardContent class="p-0">
				{#if loading}
					<div class="p-6 text-center text-muted-foreground">Loading...</div>
				{:else if runningModels.length > 0}
					{#each runningModels as model, i (model.digest || model.name)}
						<RunningModelRow {model} />
						{#if i < runningModels.length - 1}
							<Separator />
						{/if}
					{/each}
				{:else}
					<div class="p-6 text-center">
						<Zap class="h-12 w-12 mx-auto text-muted-foreground mb-4" />
						<h3 class="text-lg font-semibold">No models are currently running</h3>
						<p class="text-muted-foreground">
							Start a chat or generate text to load a model into memory
						</p>
					</div>
				{/if}
			</CardContent>
		</Card>
	</div>
</div>