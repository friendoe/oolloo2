<script lang="ts">
	import { cn } from '$lib/utils.js';
	import { badgeVariants, type Props } from './index.js';

	let {
		variant = 'default',
		class: className = undefined,
		...rest
	}: Props & { class?: string } = $props();
</script>

<div class={cn(badgeVariants({ variant }), className)} {...rest}>
	<slot />
</div>