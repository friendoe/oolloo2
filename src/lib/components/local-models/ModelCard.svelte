<script lang="ts">
	import {
		Card,
		CardContent,
		CardDescription,
		CardFooter,
		CardHeader,
		CardTitle
	} from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Trash2 } from '@lucide/svelte';
	import { Badge } from '$lib/components/ui/badge';

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

	let { model, onDelete }: { model: Model; onDelete: (name: string) => void } = $props();

	let deleting = $state(false);

	function formatBytes(bytes: number, decimals = 2) {
		if (bytes === 0) return '0 Bytes';
		const k = 1024;
		const dm = decimals < 0 ? 0 : decimals;
		const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
	}

	async function deleteModel() {
		if (deleting) return;

		const confirmed = confirm(`Are you sure you want to delete the model "${model.name}"?`);
		if (!confirmed) return;

		deleting = true;
		try {
			const response = await fetch('http://localhost:11434/api/delete', {
				method: 'DELETE',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ name: model.name })
			});

			if (!response.ok) {
				const errorText = await response.text();
				throw new Error(`Failed to delete model: ${errorText}`);
			}

			// Notify parent component that deletion was successful
			onDelete(model.name);
		} catch (error: any) {
			console.error(error);
			alert(error.message);
		} finally {
			deleting = false;
		}
	}
</script>

<Card class="flex flex-col h-full">
	<CardHeader>
		<CardTitle class="text-lg">{model.name.split(':')[0]}</CardTitle>
		<CardDescription>
			<Badge variant="secondary">{model.name.split(':')[1] || 'latest'}</Badge>
		</CardDescription>
	</CardHeader>
	<CardContent class="flex-grow">
		{#if model.details?.family}
			<p class="text-sm text-muted-foreground">
				{model.details.family} family
			</p>
		{/if}
		<p class="text-sm text-muted-foreground mt-2">
			Size: {formatBytes(model.size)}
		</p>
	</CardContent>
	<CardFooter>
		<Button variant="destructive" onclick={deleteModel} disabled={deleting} class="w-full">
			{#if deleting}
				<svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
					<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
					<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
				</svg>
				Deleting...
			{:else}
				<Trash2 class="mr-2 h-4 w-4" />
				Delete
			{/if}
		</Button>
	</CardFooter>
</Card>