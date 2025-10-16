<script lang="ts">
	import { cn } from '$lib/utils.js';
	import { alertVariants, type Props } from './index.js';

	let {
		variant = 'default',
		class: className = undefined,
		...rest
	}: Props & { class?: string } = $props();
</script>

<div
	role="alert"
	class={cn(alertVariants({ variant }), className)}
	{...rest}
>
	<slot />
</div>