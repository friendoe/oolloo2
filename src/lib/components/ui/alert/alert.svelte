<script lang="ts">
	import { cn } from '$lib/utils.js';
	import { alertVariants, type Props } from './index.js';

	import type { Snippet } from 'svelte';

	let {
		variant = 'default',
		class: className = undefined,
		children,
		...rest
	}: Props & { class?: string; children: Snippet } = $props();
</script>

<div
	role="alert"
	class={cn(alertVariants({ variant }), className)}
	{...rest}
>
	{@render children()}
</div>