<script lang="ts">
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Box, Zap, Cpu, Database } from 'lucide-svelte';

	let {
		title,
		value,
		description
	}: {
		title: string;
		value: number | string;
		description: string;
	} = $props();

	const icons: Record<string, any> = {
		'Total Models': Box,
		'Running Models': Zap,
		'Memory Usage': Cpu,
		'Disk Usage': Database
	};

	const Icon = icons[title];
</script>

<Card>
	<CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
		<CardTitle class="text-sm font-medium">{title}</CardTitle>
		<Icon class="h-4 w-4 text-muted-foreground" />
	</CardHeader>
	<CardContent>
		<div class="text-2xl font-bold">{value}</div>
		<p class="text-xs text-muted-foreground">{description}</p>
	</CardContent>
</Card>