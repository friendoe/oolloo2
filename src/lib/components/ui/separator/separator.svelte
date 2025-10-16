<script lang="ts">
	import { cn } from '$lib/utils.js';
	import { separatorVariants, type Props } from './index.js';

	let {
		orientation = 'horizontal',
		class: className = undefined,
		...rest
	}: Props & { class?: string } = $props();
</script>

<div
	class={cn(separatorVariants({ orientation }), className)}
	role="separator"
	aria-orientation={orientation === 'vertical' ? 'vertical' : 'horizontal'}
	{...rest}
/>