<script lang="ts">
	import { cn } from '$lib/utils.js';
	import { badgeVariants, type Props } from './index.js';

	import type { Snippet } from 'svelte';

	let {
		variant = 'default',
		class: className = undefined,
		children,
		...rest
	}: Props & { class?: string; children: Snippet } = $props();
</script>

<div class={cn(badgeVariants({ variant }), className)} {...rest}>
	{@render children()}
</div>